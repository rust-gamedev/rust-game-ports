use fyrox::gui::{message::MessageDirection, widget::WidgetMessage, UiNode, UserInterface};

use crate::prelude::*;

const BAR_IMG_BASE: &str = "bar";
const SCORE_IMG_BASE: &str = "s";
const GOAL_IMG_BASE: &str = "goal";

pub struct GameHud {
    bar_h: Handle<UiNode>,
    score_hs: [Handle<UiNode>; 2],
    goal_h: Handle<UiNode>,
}

impl GameHud {
    pub fn new(user_interface: &mut UserInterface, media: &Media) -> Self {
        let bar_h = add_widget_node(
            media,
            BAR_IMG_BASE,
            &[],
            HALF_WINDOW_W - 176.,
            0.,
            user_interface,
        );

        let score_hs = (0..2)
            .map(|i| {
                add_widget_node(
                    media,
                    SCORE_IMG_BASE,
                    &[0],
                    HALF_WINDOW_W + 7. - 39. * (i as f32),
                    6.,
                    user_interface,
                )
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let goal_h = add_widget_node(
            media,
            GOAL_IMG_BASE,
            &[],
            HALF_WINDOW_W - 300.,
            HEIGHT / 2. - 88.,
            user_interface,
        );

        Self {
            bar_h,
            score_hs,
            goal_h,
        }
    }

    pub fn prepare_draw(
        &self,
        team_scores: &[u8],
        display_goal: bool,
        media: &Media,
        user_interface: &mut UserInterface,
    ) {
        enable_widget(
            self.bar_h,
            media,
            BAR_IMG_BASE,
            &[],
            HALF_WINDOW_W - 176.,
            0.,
            user_interface,
        );

        for (i, score_h) in self.score_hs.iter().enumerate() {
            enable_widget(
                *score_h,
                media,
                SCORE_IMG_BASE,
                &[team_scores[i]],
                HALF_WINDOW_W + 7. - 39. * (i as f32),
                6.,
                user_interface,
            );
        }

        if display_goal {
            enable_widget(
                self.goal_h,
                media,
                GOAL_IMG_BASE,
                &[],
                HALF_WINDOW_W - 300.,
                HEIGHT / 2. - 88.,
                user_interface,
            );
        } else {
            disable_widget(self.goal_h, user_interface);
        }
    }

    pub fn clear(&self, user_interface: &mut UserInterface) {
        user_interface.send_message(WidgetMessage::remove(
            self.bar_h,
            MessageDirection::ToWidget,
        ));
        for score_h in self.score_hs {
            user_interface.send_message(WidgetMessage::remove(score_h, MessageDirection::ToWidget));
        }
        user_interface.send_message(WidgetMessage::remove(
            self.goal_h,
            MessageDirection::ToWidget,
        ));
    }
}
