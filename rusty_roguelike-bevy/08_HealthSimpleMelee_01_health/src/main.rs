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
        let map_builder = MapBuilder::new(&mut rng);
        // This is not a strict-ECS approach (a system would), but we mimick the source project design.
        spawn_player(&mut ecs.world, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(bracket_lib::prelude::Rect::center)
            .for_each(|pos| spawn_monster(&mut ecs.world, &mut rng, pos));
        ecs.insert_resource(map_builder.map);
        ecs.insert_resource(Camera::new(map_builder.player_start));
        // In Bevy, it's necessary to register the event types.
        ecs.add_event::<WantsToMove>();
        // Set the additional stages
        ecs.add_stage_after(CoreStage::Update, MovePlayer, SystemStage::parallel())
            .add_stage_after(MovePlayer, Collisions, SystemStage::parallel())
            .add_stage_after(Collisions, GenerateMonsterMoves, SystemStage::parallel())
            .add_stage_after(GenerateMonsterMoves, MoveMonsters, SystemStage::parallel());
        // Set the startup state.
        ecs.add_loopless_state(TurnState::AwaitingInput);
        // In the source project, set of actions (`Schedule`s) are owned by State (`systems: Schedule`);
        // here, they're owned by the Bevy ECS, as `SystemSet`s.
        build_system_sets(&mut ecs);
        Self { ecs }
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
