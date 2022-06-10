use fyrox::scene::SceneContainer;
use strum::IntoEnumIterator;

use crate::prelude::*;

// Like Pools, this makes it more compact to store the scenes.
pub struct Scenes {
    pub menu: Handle<Scene>,
    play: Handle<Scene>,
    game_over: Handle<Scene>,
}

impl Scenes {
    pub fn new(scene_container: &mut SceneContainer) -> Self {
        let menu = Self::add_new_disabled_scene(scene_container);
        let play = Self::add_new_disabled_scene(scene_container);
        let game_over = Self::add_new_disabled_scene(scene_container);

        Self {
            menu,
            play,
            game_over,
        }
    }

    pub fn scene<'a>(
        &self,
        state: State,
        scene_container: &'a mut SceneContainer,
    ) -> &'a mut Scene {
        use crate::state::State::*;

        match state {
            Menu => scene_container.try_get_mut(self.menu).unwrap(),
            Play => scene_container.try_get_mut(self.play).unwrap(),
            GameOver => scene_container.try_get_mut(self.game_over).unwrap(),
        }
    }

    // Disables the other states.
    pub fn enable(&self, enable_state: State, scene_container: &mut SceneContainer) {
        for state in State::iter() {
            self.scene(state, scene_container).enabled = state == enable_state;
        }
    }

    fn add_new_disabled_scene(scene_container: &mut SceneContainer) -> Handle<Scene> {
        let mut scene = Scene::new();
        scene.enabled = false;
        scene_container.add(scene)
    }
}
