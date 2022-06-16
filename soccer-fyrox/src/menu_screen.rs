use fyrox::gui::{UiNode, UserInterface};

use crate::prelude::*;

const IMG_BASE: &str = "menu";

pub struct MenuScreen {
    widget_h: Handle<UiNode>,
}

impl MenuScreen {
    // Starts with 1 player selected.
    //
    pub fn new(user_interface: &mut UserInterface, media: &Media) -> Self {
        let widget_h = Handle::NONE;

        let mut instance = Self { widget_h };

        instance.display(&[0, 1], media, user_interface);

        instance
    }

    pub fn display(&mut self, indexes: &[u8], media: &Media, user_interface: &mut UserInterface) {
        self.widget_h = add_widget_node(media, IMG_BASE, &[0, 1], 0., 0., user_interface);
        self.update_selection(indexes, media, user_interface);
    }

    pub fn update_selection(
        &self,
        indexes: &[u8],
        media: &Media,
        user_interface: &mut UserInterface,
    ) {
        update_widget_texture(self.widget_h, media, IMG_BASE, indexes, user_interface);
    }

    pub fn clear(&mut self, user_interface: &mut UserInterface) {
        self.widget_h = remove_widget_node(self.widget_h, user_interface);
    }
}
