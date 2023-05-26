use dbus::arg;

use super::dbus_server::engine::IBusEngine;

pub struct FcpEngine {}

impl IBusEngine for FcpEngine {
    fn process_key_event(&mut self, keyval: u32, keycode: u32, state: u32) -> Result<bool, dbus::MethodErr> {
        unimplemented!();
    }
    fn set_cursor_location(&mut self, x_: i32, y_: i32, w_: i32, h_: i32) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn process_hand_writing_event(&mut self, coordinates: Vec<f64>) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn cancel_hand_writing(&mut self, n_strokes: u32) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn set_capabilities(&mut self, caps: u32) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn property_activate(&mut self, name: String, state: u32) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn property_show(&mut self, name: String) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn property_hide(&mut self, name: String) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn candidate_clicked(&mut self, index: u32, button: u32, state: u32) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn focus_in(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn focus_in_id(&mut self, object_path: String, client: String) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn focus_out(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn focus_out_id(&mut self, object_path: String) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn reset(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn enable(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn disable(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn page_up(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn page_down(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn cursor_up(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn cursor_down(&mut self) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn set_surrounding_text(&mut self, text: arg::Variant<Box<dyn arg::RefArg + 'static>>, cursor_pos: u32, anchor_pos: u32) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn panel_extension_received(&mut self, event: arg::Variant<Box<dyn arg::RefArg + 'static>>) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn panel_extension_register_keys(&mut self, data: arg::Variant<Box<dyn arg::RefArg + 'static>>) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn set_content_type(&self, value: (u32, u32)) -> Result<(), dbus::MethodErr> {
        unimplemented!();
    }
    fn focus_id(&self) -> Result<(bool), dbus::MethodErr> {
        unimplemented!();
    }
    fn active_surrounding_text(&self) -> Result<(bool), dbus::MethodErr> {
        unimplemented!();
    }
}