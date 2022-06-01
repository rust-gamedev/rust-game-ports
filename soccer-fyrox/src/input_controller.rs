use std::collections::HashMap;

use crate::prelude::*;

#[derive(Default)]
pub struct InputController {
    // The value is a tuple of previous and last state (true = pressed).
    // Once an entry is added, it's never removed - on key released, the value is set as (false, false).
    //
    key_states: HashMap<VirtualKeyCode, (bool, bool)>,
    // Fyrox doesn't expose input handling APIs in the `on_tick()` function; since in some cases (key
    // kept pressed) it can take several frames to receive the next event, we need a way to understand
    // how to interpret the state in between - we do it through this variable; see `is_key_just_pressed()`.
    //
    event_received: bool,
}

// WATCH OUT!!! It's **crucial** to invoke `flush_event_received_state()` at the end of `on_tick()`,
// otherwise, the "just pressed" functionality won't work.
//
impl InputController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn flush_event_received_state(&mut self) {
        self.event_received = false
    }

    pub fn key_down(&mut self, key: VirtualKeyCode) {
        self.key_states
            .entry(key)
            .and_modify(|v| *v = (v.1, true))
            // If it wasn't tracked, we assume that it was not pressed before tracking started.
            .or_insert((false, true));

        self.event_received = true;
    }

    pub fn key_up(&mut self, key: VirtualKeyCode) {
        self.key_states
            .entry(key)
            .and_modify(|v| *v = (v.1, false))
            // If it wasn't tracked, we assume that it was pressed before tracking started.
            .or_insert((true, false));

        self.event_received = true;
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        let key_state = self.key_states.get(&key).unwrap_or(&(false, false));
        key_state.1
    }

    // Although is_key_pressed would also do in some cases, e.g. menus, this is still a user-friendly
    // choice, since the alternate API may generate too many keystrokes.
    //
    pub fn is_key_just_pressed(&self, key: VirtualKeyCode) -> bool {
        let key_state = self.key_states.get(&key).unwrap_or(&(false, false));

        let (previously_pressed, currently_pressed) = *key_state;

        // This logic can be compacted, however, it's kept extended for clarity.
        //
        if !currently_pressed {
            false
        } else {
            if !self.event_received {
                // If no events have been received, we assume that the state has not changed; for this
                // reason, "just pressed" is necessarily false.
                //
                false
            } else {
                !previously_pressed && currently_pressed
            }
        }
    }
}
