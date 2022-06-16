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

        instance.display(media, user_interface);

        instance
    }

    pub fn display(&mut self, media: &Media, user_interface: &mut UserInterface) {
        self.widget_h = add_widget_node(0., 0., user_interface);
        self.update_selection(MenuState::NumPlayers, 1, 1, media, user_interface);
    }

    pub fn update_selection(
        &self,
        menu_state: MenuState,
        menu_num_players: u8,
        menu_difficulty: u8,
        media: &Media,
        user_interface: &mut UserInterface,
    ) {
        use MenuState::*;

        let image_indexes = match menu_state {
            NumPlayers => [0, menu_num_players],
            Difficulty => [1, menu_difficulty],
        };

        update_widget_texture(
            self.widget_h,
            media,
            IMG_BASE,
            &image_indexes,
            user_interface,
        );
    }

    pub fn clear(&mut self, user_interface: &mut UserInterface) {
        self.widget_h = remove_widget_node(self.widget_h, user_interface);
    }
}
