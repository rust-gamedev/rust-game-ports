use tetra::{
    graphics::{DrawParams, Texture},
    math::Vec2,
    Context, State as TetraState, TetraError,
};

use crate::{
    bunner::Bunner, eagle::Eagle, grass::Grass, resource_path::resource_path, row::Row,
    state::GameState, HEIGHT, WIDTH,
};

pub struct Game {
    high_score: u32,
    state: GameState,
    _bunner: Option<Bunner>,
    _eagle: Option<Eagle>,
    _frame: u32,
    _rows: Vec<Box<dyn Row>>,
    scroll_pos: i32,

    title_texture: Texture,
    start_textures: Vec<Texture>,
    gameover_texture: Texture,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> tetra::Result<Self> {
        let _rows: Vec<Box<dyn Row>> = vec![Box::new(Grass::new(None, 0, 0))];
        let scroll_pos = -HEIGHT;

        let title_texture = Texture::new(_ctx, "resources/images/title.png")?;
        let start_textures = (0..3)
            .map(|i| Texture::new(_ctx, format!("resources/images/start{}.png", i)).unwrap())
            .collect();
        let gameover_texture = Texture::new(_ctx, "resources/images/gameover.png")?;

        Ok(Self {
            high_score: 0,
            state: GameState::MENU,
            _bunner: None,
            _eagle: None,
            _frame: 0,
            _rows,
            scroll_pos,

            title_texture,
            start_textures,
            gameover_texture,
        })
    }

    fn score(&self) -> u32 {
        println!("WRITEME: score");
        0
    }

    fn display_number(&self, _n: u32, _colour: u32, _x: i32, _align: u32) {
        println!("WRITEME: display_number")
    }
}

impl TetraState for Game {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        // game.draw() goes here

        match self.state {
            GameState::MENU => {
                self.title_texture
                    .draw(ctx, DrawParams::new().position(Vec2::new(0., 0.)));

                // In Python, (-n // 6) has different semantics from Rust, for negative numbers, so we
                // adjust. If we don't do this, in the interval (-6, +5), a single image is printed,
                // since the division returns 0.
                //
                let start_texture_i = (self.scroll_pos as f32 / 6.).floor().rem_euclid(4.) as usize;
                let start_texture = [0, 1, 2, 1][start_texture_i];

                self.start_textures[start_texture].draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new((WIDTH as f32 - 270.) / 2., HEIGHT as f32 - 240.)),
                );
            }
            GameState::PLAY => {
                self.display_number(self.score(), 0, 0, 0);
                self.display_number(self.high_score, 1, WIDTH - 10, 1);
            }
            GameState::GAME_OVER => {
                self.gameover_texture
                    .draw(ctx, DrawParams::new().position(Vec2::new(0., 0.)));
            }
        };

        Ok(())
    }
}
