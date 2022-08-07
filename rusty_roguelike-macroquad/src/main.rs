mod camera_view;
mod components;
mod macroquad_utils;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref RANDOM_FLOOR_TILES: Vec<Sprite> = gen_random_tiles(0, 16);
    pub static ref RANDOM_TREE_TILES: Vec<Sprite> = gen_random_tiles(16, 20);
}

fn gen_random_tiles(lower: usize, upper: usize) -> Vec<Sprite> {
    (0..=(SCREEN_WIDTH * SCREEN_HEIGHT))
        .map(|_| rand::gen_range(lower, upper) as Sprite)
        .collect()
}

mod prelude {
    pub use bracket_pathfinding::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use macroquad::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera_view::*;
    pub use crate::components::*;
    pub use crate::macroquad_utils::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::RANDOM_FLOOR_TILES;
    pub use crate::RANDOM_TREE_TILES;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    texture: Texture2D,
}

impl State {
    async fn new(texture: Texture2D) -> Self {
        rand::srand(miniquad::date::now() as u64);
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut map_builder = MapBuilder::new();
        let tileset = Self::tileset(texture);
        spawn_player(&mut ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&mut ecs, 0, &map_builder.monster_spawns).await;
        resources.insert(map_builder.map);
        resources.insert(CameraView::new(map_builder.player_start));
        resources.insert(tileset);
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            texture,
        }
    }

    fn tileset(texture: Texture2D) -> TileSet {
        TileSet {
            texture: texture,
            tile_width: 32,
            tile_height: 32,
            columns: 16,
        }
    }

    async fn tick(&mut self) {
        clear_background(BLACK);
        self.resources.insert(get_last_key_pressed());
        self.resources
            .insert(Point::from_tuple(mouse_tile_position()));
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over().await,
            TurnState::Victory => self.victory().await,
            TurnState::NextLevel => self.advance_level().await,
        }
    }

    async fn game_over(&mut self) {
        print_color_centered(2, "Your quest has ended.", RED);
        print_color_centered(
            4,
            "Slain by a monster, your hero's journey has come to a \
            premature end.",
            WHITE,
        );
        print_color_centered(
            5,
            "The Amulet of YALA remains unclaimed, and your home town \
            is not saved.",
            WHITE,
        );
        print_color_centered(
            8,
            "Don't worry, you can always try again with a new hero.",
            YELLOW,
        );
        print_color_centered(9, "Press 1 to play again.", GREEN);

        if is_key_down(KeyCode::Key1) {
            self.reset_game_state().await;
        }
    }

    async fn victory(&mut self) {
        print_color_centered(2, "You have won!", GREEN);
        print_color_centered(
            4,
            "You put on the Amulet of YALA and feel its power course through \
            your veins.",
            WHITE,
        );
        print_color_centered(
            5,
            "Your town is saved, and you can return to your normal life.",
            WHITE,
        );
        print_color_centered(7, "Press 1 to play again.", GREEN);

        if is_key_down(KeyCode::Key1) {
            self.reset_game_state().await;
        }
    }

    async fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();

        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut map_builder = MapBuilder::new();
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(
            &mut self.ecs,
            map_level as usize,
            &map_builder.monster_spawns,
        )
        .await;
        self.resources.insert(map_builder.map);
        self.resources
            .insert(CameraView::new(map_builder.player_start));
        let tileset = Self::tileset(self.texture);
        self.resources.insert(tileset);
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    async fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut map_builder = MapBuilder::new();
        let tileset = Self::tileset(self.texture);
        spawn_player(&mut self.ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&mut self.ecs, 0, &map_builder.monster_spawns).await;
        self.resources.insert(map_builder.map);
        self.resources
            .insert(CameraView::new(map_builder.player_start));
        self.resources.insert(tileset);
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Dungeon".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let tileset = load_texture("assets/dungeonfont.png")
        .await
        .expect("Tile texture not found");
    tileset.set_filter(FilterMode::Nearest);

    let mut game = State::new(tileset).await;
    loop {
        game.tick().await;
        next_frame().await
    }
}
