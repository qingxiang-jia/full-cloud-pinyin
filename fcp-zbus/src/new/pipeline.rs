use super::mode_switcher::{self, ModeSwitcher, ModeSwitcherReturn};

pub struct Pipeline {
    mode_switcher: ModeSwitcher,
}

impl Pipeline {
    pub async fn accept(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        let output = self
            .mode_switcher
            .process_key_event(keyval, keycode, state)
            .await;
        if output.is_done() {
            return output
                .get_done_data()
                .expect("ModeSwitcherReturn is Done but doesn't have data.");
        }

        let (key, should_reset) = output
            .get_continue_data()
            .expect("ModeSwitcherReturn is Continue but doesn't have data.");

        return false;
    }
}
