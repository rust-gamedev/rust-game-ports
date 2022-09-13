use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use sdl2::{
    mouse::MouseUtil, AudioSubsystem, EventPump, EventSubsystem, GameControllerSubsystem,
    JoystickSubsystem, Sdl, TimerSubsystem, VideoSubsystem,
};

#[derive(Clone)]
pub struct SdlManager {
    /// The Rc is necessary in order to be used in Sdl events.
    sdl: Rc<Option<Sdl>>,
    // The following need to stay in scope (at least currently).
    audio: AudioSubsystem,
    joystick: JoystickSubsystem,
    game_controller: GameControllerSubsystem,
    // This needs to stay in scope because there can be only one.
    event_pump: Rc<RefCell<EventPump>>,
}

impl SdlManager {
    pub fn init_sdl() -> Self {
        let sdl = sdl2::init().expect("Failed to initialize SDL");

        let audio = sdl.audio().unwrap();
        let joystick = sdl.joystick().unwrap();
        let game_controller = sdl.game_controller().unwrap();
        let event_pump = Rc::new(RefCell::new(sdl.event_pump().unwrap()));

        Self {
            sdl: Rc::new(Some(sdl)),
            audio,
            joystick,
            game_controller,
            event_pump,
        }
    }
}

impl SdlManager {
    pub fn video(&self) -> VideoSubsystem {
        self.sdl().video().unwrap()
    }

    pub fn audio(&self) -> &AudioSubsystem {
        &self.audio
    }

    pub fn timer(&self) -> TimerSubsystem {
        self.sdl().timer().unwrap()
    }

    pub fn joystick(&self) -> &JoystickSubsystem {
        &self.joystick
    }

    pub fn game_controller(&self) -> &GameControllerSubsystem {
        &self.game_controller
    }

    pub fn event(&self) -> EventSubsystem {
        self.sdl().event().unwrap()
    }

    pub fn mouse(&self) -> MouseUtil {
        self.sdl().mouse()
    }

    pub fn event_pump(&self) -> RefMut<EventPump> {
        (*self.event_pump).borrow_mut()
    }

    pub fn quit(&mut self) {
        self.sdl = Rc::new(None);
    }

    fn sdl(&self) -> &Sdl {
        (*self.sdl).as_ref().unwrap()
    }
}
