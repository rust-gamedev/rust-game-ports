use fyrox::gui::{UiNode, UserInterface};

use crate::prelude::*;

const BAR_IMG_BASE: &str = "bar";
const SCORE_IMG_BASE: &str = "s";
const GOAL_IMG_BASE: &str = "goal";

pub struct GameHud {
    bar_h: Handle<UiNode>,
    score_hs: Vec<Handle<UiNode>>,
    goal_h: Handle<UiNode>,
}

impl GameHud {
    // Doesn't display the screen or perform any instantiation.
    //
    pub fn new() -> Self {
        let bar_h = Handle::NONE;
        let score_hs = vec![Handle::NONE, Handle::NONE];
        let goal_h = Handle::NONE;

        Self {
            bar_h,
            score_hs,
            goal_h,
        }
    }

    pub fn display(&mut self, media: &Media, user_interface: &mut UserInterface) {
        self.bar_h = add_widget_node(HALF_WINDOW_W - 176., 0., user_interface);
        update_widget_texture(self.bar_h, media, BAR_IMG_BASE, &[], user_interface);

        for (i, score_h) in self.score_hs.iter_mut().enumerate() {
            *score_h = add_widget_node(HALF_WINDOW_W + 7. - 39. * (i as f32), 6., user_interface);
        }

        self.goal_h = add_widget_node(HALF_WINDOW_W - 300., HEIGHT / 2. - 88., user_interface);
        update_widget_texture(self.goal_h, media, GOAL_IMG_BASE, &[], user_interface);

        self.update(&[0, 0], false, media, user_interface);
    }

    pub fn update(
        &mut self,
        team_scores: &[u8],
        display_goal: bool,
        media: &Media,
        user_interface: &mut UserInterface,
    ) {
        for (i, score_h) in self.score_hs.iter_mut().enumerate() {
            update_widget_texture(
                *score_h,
                media,
                SCORE_IMG_BASE,
                &[team_scores[i]],
                user_interface,
            );
        }

        if display_goal {
            enable_widget_node(self.goal_h, user_interface);
        } else {
            disable_widget_node(self.goal_h, user_interface);
        }
    }

    pub fn clear(&mut self, user_interface: &mut UserInterface) {
        self.bar_h = remove_widget_node(self.bar_h, user_interface);

        for score_h in &mut self.score_hs {
            *score_h = remove_widget_node(*score_h, user_interface);
        }

        self.goal_h = remove_widget_node(self.goal_h, user_interface);
    }
}
