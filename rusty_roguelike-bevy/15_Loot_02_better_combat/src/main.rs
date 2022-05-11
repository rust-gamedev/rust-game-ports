mod camera;
mod components;
mod events;
mod game_stage;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    // Keep a space, in order to prevent IDEs to reorder imports, which causes clashing.
    pub use bevy::prelude::*;
    pub use iyes_loopless::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::game_stage::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: App,
}

impl State {
    fn new() -> Self {
        use game_stage::GameStage::*;

        let mut ecs = App::new();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        // This is not a strict-ECS approach (a system would), but we mimick the source project design.
        spawn_player(&mut ecs.world, map_builder.player_start);
        //spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&mut ecs.world, &mut rng, 0, &map_builder.monster_spawns);
        ecs.insert_resource(map_builder.map);
        ecs.insert_resource(Camera::new(map_builder.player_start));
        // In Bevy, it's necessary to register the event types.
        ecs.add_event::<WantsToMove>();
        ecs.add_event::<WantsToAttack>();
        ecs.add_event::<ActivateItem>();
        // Set the additional stages
        ecs.add_stage_after(CoreStage::Update, PlayerCombat, SystemStage::parallel())
            .add_stage_after(PlayerCombat, MovePlayer, SystemStage::parallel())
            .add_stage_after(MovePlayer, PlayerFov, SystemStage::parallel())
            .add_stage_after(PlayerFov, GenerateMonsterMoves, SystemStage::parallel())
            .add_stage_after(GenerateMonsterMoves, MonsterCombat, SystemStage::parallel())
            .add_stage_after(MonsterCombat, MoveMonsters, SystemStage::parallel())
            .add_stage_after(MoveMonsters, MonsterFov, SystemStage::parallel());
        // Set the startup state.
        ecs.add_loopless_state(TurnState::AwaitingInput);
        ecs.insert_resource(map_builder.theme);
        // In the source project, set of actions (`Schedule`s) are owned by State (`systems: Schedule`);
        // here, they're owned by the Bevy ECS, as `SystemSet`s.
        build_system_sets(&mut ecs);
        // This is the way used by App#run to get ownership of the App instance.
        ecs = std::mem::replace(&mut ecs, App::empty());
        Self { ecs }
    }

    fn reset_game_state(&mut self) {
        // We can't reset the world like Legion, because it's not supported by iyes_loopless.
        // Resources clearing is actually tricky, because Bevy/plugins may have their own resources,
        // whose deletion may not be exposed. One example is events; we get away without clearing
        // them, because by the time GameOver is reached, the events are all consumed.
        // Regarding `iyes_loopless`, we don't need to take care of it; we just set the next state.
        // Finally, the resources directly known to us, we just overwrite them.
        // Note that we can also just replace the current app with a new one.
        self.ecs.world.clear_entities();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs.world, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(
            &mut self.ecs.world,
            &mut rng,
            0,
            &map_builder.monster_spawns,
        );
        self.ecs.insert_resource(map_builder.map);
        self.ecs
            .insert_resource(Camera::new(map_builder.player_start));
        self.ecs
            .insert_resource(NextState(TurnState::AwaitingInput));
        self.ecs.insert_resource(map_builder.theme);
        // Don't forget! :)
        self.ecs.world.remove_resource::<VirtualKeyCode>();
    }

    fn advance_level(&mut self) {
        let mut player_query = self.ecs.world.query_filtered::<Entity, With<Player>>();
        let player_entity = player_query.iter(&self.ecs.world).next().unwrap();

        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        let mut carry_query = self.ecs.world.query::<(Entity, &Carried)>();
        for (e, carry) in carry_query.iter(&self.ecs.world) {
            if carry.0 == player_entity {
                entities_to_keep.insert(e);
            }
        }
        let mut entities_query = self.ecs.world.query::<Entity>();
        // In Bevy, we can't query the world and write to it at the same time, so we need an intermediate
        // collection.
        // We're using a convenient Rust construct: filter_map() + bool#then, in order to compactly
        // filter an iterator.
        let entities_to_remove = entities_query
            .iter(&self.ecs.world)
            .filter_map(|e| (!entities_to_keep.contains(&e)).then(|| e))
            .collect::<Vec<_>>();
        // We don't need flushing; when manipulating World directly in Bevy, flushes are implicit.
        for e in entities_to_remove {
            self.ecs.world.despawn(e);
        }

        let mut fov_query = self.ecs.world.query::<&mut FieldOfView>();
        for mut fov in fov_query.iter_mut(&mut self.ecs.world) {
            fov.is_dirty = true;
        }

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        let mut player_query = self.ecs.world.query::<(&mut Player, &mut PointC)>();
        for (mut player, mut pos) in player_query.iter_mut(&mut self.ecs.world) {
            player.map_level += 1;
            map_level = player.map_level;
            pos.0.x = map_builder.player_start.x;
            pos.0.y = map_builder.player_start.y;
        }
        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs.world, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }
        spawn_level(
            &mut self.ecs.world,
            &mut rng,
            0,
            &map_builder.monster_spawns,
        );
        self.ecs.world.insert_resource(map_builder.map);
        self.ecs
            .world
            .insert_resource(Camera::new(map_builder.player_start));
        self.ecs
            .insert_resource(NextState(TurnState::AwaitingInput));
        self.ecs.world.insert_resource(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a \
            premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town \
            is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through \
            your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(
            7,
            GREEN,
            BLACK,
            "Press 1 to \
            play again.",
        );
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        if let Some(key) = ctx.key {
            self.ecs.insert_resource(key);
        } else {
            // In order to keep consistency with the Legion version, we need to access Bevy's World
            // directly, since App doesn't support removing resources.
            self.ecs.world.remove_resource::<VirtualKeyCode>();
        }
        ctx.set_active_console(0);
        self.ecs.insert_resource(Point::from_tuple(ctx.mouse_pos()));
        // Unfortunately, with the current source project's design, without refactoring the world init
        // code into systems, we must leak the state into this abstraction.
        match self.ecs.world.get_resource::<CurrentState<TurnState>>() {
            Some(CurrentState(TurnState::GameOver)) => self.game_over(ctx),
            Some(CurrentState(TurnState::Victory)) => self.victory(ctx),
            Some(CurrentState(TurnState::NextLevel)) => self.advance_level(),
            _ => {}
        }
        self.ecs.update();
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
