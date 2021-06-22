use macroquad::{
    audio::{self, Sound},
    rand::ChooseRandom,
};

use crate::player::Player;

// Utility function for game audio playback.
//
// The second one is separate due to a current architectural shortcoming - in the original code, Game
// is global, so this is a simple implementation before the redesign.
// The first one fits in Game, but it would be ugly to have them in different locations.

pub fn play_game_sound(player: Option<&Player>, sound: &Sound) {
    if player.is_some() {
        audio::play_sound_once(*sound);
    }
}

pub fn play_game_random_sound(player: Option<&Player>, sounds: &Vec<Sound>) {
    play_game_sound(player, sounds.choose().unwrap())
}
