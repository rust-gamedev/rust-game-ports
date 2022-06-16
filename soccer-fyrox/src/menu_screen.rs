use fyrox::gui::{UiNode, UserInterface};

use crate::prelude::*;

const IMG_BASE: &str = "menu";

pub struct MenuScreen {
    widget_h: Handle<UiNode>,
}

impl MenuScreen {
    pub fn new(user_interface: &mut UserInterface, media: &Media) -> Self {
        let widget_h = add_widget_node(media, IMG_BASE, &[0, 1], 0., 0., user_interface);

        Self { widget_h }
    }

    pub fn prepare_draw(
        &mut self,
        indexes: &[u8],
        media: &Media,
        user_interface: &mut UserInterface,
    ) {
        self.widget_h = add_widget_node(media, IMG_BASE, indexes, 0., 0., user_interface);
    }

    pub fn clear(&self, user_interface: &mut UserInterface) {
        remove_widget_node(self.widget_h, user_interface);
    }
}
