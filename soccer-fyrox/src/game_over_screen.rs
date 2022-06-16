use fyrox::gui::{UiNode, UserInterface};

use crate::prelude::*;

const BACKGROUND_IMG_BASE: &str = "over";
const SCORE_IMG_BASE: &str = "l";

pub struct GameOverScreen {
    background_h: Handle<UiNode>,
    score_hs: [Handle<UiNode>; 2],
}

impl GameOverScreen {
    pub fn new(user_interface: &mut UserInterface, media: &Media) -> Self {
        let background_h =
            add_widget_node(media, BACKGROUND_IMG_BASE, &[0], 0., 0., user_interface);

        let score_hs = (0..2)
            .map(|i| {
                add_widget_node(
                    media,
                    SCORE_IMG_BASE,
                    &[0, 0],
                    HALF_WINDOW_W + 25. - 125. * i as f32,
                    144.,
                    user_interface,
                )
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            background_h,
            score_hs,
        }
    }

    pub fn prepare_draw(
        &mut self,
        background_index: u8,
        team_scores: &[u8],
        media: &Media,
        user_interface: &mut UserInterface,
    ) {
        self.background_h = add_widget_node(
            media,
            BACKGROUND_IMG_BASE,
            &[background_index],
            0.,
            0.,
            user_interface,
        );

        for (i, (score_h, team_score)) in
            self.score_hs.iter_mut().zip(team_scores.iter()).enumerate()
        {
            *score_h = add_widget_node(
                media,
                SCORE_IMG_BASE,
                &[i as u8, *team_score],
                HALF_WINDOW_W + 25. - 125. * i as f32,
                144.,
                user_interface,
            );
        }
    }

    pub fn clear(&self, user_interface: &mut UserInterface) {
        remove_widget_node(self.background_h, user_interface);
        for score_h in self.score_hs {
            remove_widget_node(score_h, user_interface);
        }
    }
}
