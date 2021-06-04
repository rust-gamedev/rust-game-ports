// Simplistic design; very similar considerations to the control functions apply.

use ggez::{
    audio::{self, SoundSource},
    Context, GameResult,
};

use crate::state::State;

pub fn play_in_game_sound(
    context: &mut Context,
    state: State,
    sound: &mut audio::Source,
) -> GameResult {
    if state != State::Menu {
        sound.play(context)
    } else {
        Ok(())
    }
}

pub fn play_in_game_random_sound(
    context: &mut Context,
    state: State,
    sounds_collection: &mut [audio::Source],
) -> GameResult {
    let sound_i = fastrand::usize(..sounds_collection.len());
    play_in_game_sound(context, state, &mut sounds_collection[sound_i])
}
