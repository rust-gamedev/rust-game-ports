use fyrox::gui::{message::MessageDirection, widget::WidgetMessage, UiNode, UserInterface};

use crate::prelude::*;

const IMG_BASE: &str = "menu";

pub struct MenuScreen {
    widget_h: Handle<UiNode>,
}

impl MenuScreen {
    pub fn new(user_interface: &mut UserInterface, media: &Media) -> Self {
        let widget_h = build_blank_widget(media, IMG_BASE, &[0, 1], 0., 0., user_interface);

        Self { widget_h }
    }

    pub fn draw(&self, indexes: &[u8], media: &Media, user_interface: &mut UserInterface) {
        draw_widget(
            self.widget_h,
            media,
            IMG_BASE,
            indexes,
            0.,
            0.,
            user_interface,
        );
    }
    pub fn clear(&self, user_interface: &mut UserInterface) {
        user_interface.send_message(WidgetMessage::remove(
            self.widget_h,
            MessageDirection::ToWidget,
        ));
    }
}
