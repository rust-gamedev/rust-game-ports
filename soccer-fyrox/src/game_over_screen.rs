use fyrox::gui::{UiNode, UserInterface};

use crate::prelude::*;

const BACKGROUND_IMG_BASE: &str = "over";
const SCORE_IMG_BASE: &str = "l";

pub struct GameOverScreen {
    background_h: Handle<UiNode>,
    score_hs: Vec<Handle<UiNode>>,
}

impl GameOverScreen {
    // Doesn't display the screen or perform any instantiation.
    //
    pub fn new() -> Self {
        let background_h = Handle::NONE;
        let score_hs = Vec::new();

        Self {
            background_h,
            score_hs,
        }
    }

    pub fn display(
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

        update_widget_texture(
            self.background_h,
            media,
            BACKGROUND_IMG_BASE,
            &[background_index],
            user_interface,
        );

        self.score_hs = team_scores
            .iter()
            .enumerate()
            .map(|(i, team_score)| {
                let widget_h = add_widget_node(
                    media,
                    SCORE_IMG_BASE,
                    &[i as u8, *team_score],
                    HALF_WINDOW_W + 25. - 125. * i as f32,
                    144.,
                    user_interface,
                );

                update_widget_texture(
                    widget_h,
                    media,
                    SCORE_IMG_BASE,
                    &[i as u8, *team_score],
                    user_interface,
                );

                widget_h
            })
            .collect();
    }

    pub fn clear(&mut self, user_interface: &mut UserInterface) {
        self.background_h = remove_widget_node(self.background_h, user_interface);

        for score_h in self.score_hs.iter_mut() {
            *score_h = remove_widget_node(*score_h, user_interface);
        }
    }
}
