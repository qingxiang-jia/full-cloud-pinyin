use zbus::Connection;

use crate::{dispatcher::Dispatcher, mode_switcher::ModeSwitcher};

pub struct Pipeline {
    mode_switcher: ModeSwitcher,
    dispatcher: Dispatcher,
}

impl Pipeline {
    pub fn new(conn: &Connection) -> Pipeline {
        Pipeline {
            mode_switcher: ModeSwitcher::new(),
            dispatcher: Dispatcher::new(conn),
        }
    }

    pub async fn accept(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        let output = self
            .mode_switcher
            .process_key_event(keyval, keycode, state)
            .await;

        match output {
            super::mode_switcher::ModeSwitcherReturn::Continue(key, should_reset) => {
                return self.dispatcher.on_input(key, should_reset).await;
            }
            super::mode_switcher::ModeSwitcherReturn::Done(has_handled) => return has_handled,
        }
    }
}
