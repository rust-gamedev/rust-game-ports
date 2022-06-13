use fyrox::gui::{message::MessageDirection, widget::WidgetMessage, UiNode, UserInterface};

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
            build_blank_widget(media, BACKGROUND_IMG_BASE, &[0], 0., 0., user_interface);

        let score_hs = (0..2)
            .map(|i| {
                build_blank_widget(
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

    pub fn draw(
        &self,
        background_index: u8,
        team_scores: &[u8],
        media: &Media,
        user_interface: &mut UserInterface,
    ) {
        draw_widget(
            self.background_h,
            media,
            BACKGROUND_IMG_BASE,
            &[background_index],
            0.,
            0.,
            user_interface,
        );

        for (i, (score_h, team_score)) in self.score_hs.iter().zip(team_scores.iter()).enumerate() {
            draw_widget(
                *score_h,
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
        user_interface.send_message(WidgetMessage::remove(
            self.background_h,
            MessageDirection::ToWidget,
        ));
        for score_h in self.score_hs {
            user_interface.send_message(WidgetMessage::remove(score_h, MessageDirection::ToWidget));
        }
    }
}
