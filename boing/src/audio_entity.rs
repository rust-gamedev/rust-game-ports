use ggez::{
    audio::{self, SoundSource},
    Context, GameResult,
};

use crate::state::State;

/// Trait for implementing audio functionality that can't have a location corresponding to the original
/// design.
pub trait AudioEntity {
    fn play_in_game_sound(
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

    fn play_in_game_random_sound(
        context: &mut Context,
        state: State,
        sounds_collection: &mut [audio::Source],
    ) -> GameResult {
        let sound_i = fastrand::usize(..sounds_collection.len());
        Self::play_in_game_sound(context, state, &mut sounds_collection[sound_i])
    }
}
