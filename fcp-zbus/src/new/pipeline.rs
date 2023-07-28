use super::mode_switcher::{self, ModeSwitcher, ModeSwitcherReturn};

pub struct Pipeline {
    mode_switcher: ModeSwitcher,
}

impl Pipeline {
    pub async fn accept(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        /* BEGIN mode switching */
        let output = self
            .mode_switcher
            .process_key_event(keyval, keycode, state)
            .await;

        let has_handled = output.get_done_data();
        if has_handled.is_some() {
            return has_handled.expect("has_handled should have value but doesn't.");
        }

        let (key, should_reset) = output
            .get_continue_data()
            .expect("ModeSwitcherReturn is Continue but doesn't have data.");
        /* END mode switching */
        return false;
    }
}