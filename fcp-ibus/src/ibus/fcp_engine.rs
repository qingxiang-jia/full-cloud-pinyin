use dbus::arg;

use super::dbus_server::engine::IBusEngine;
use super::dbus_server::factory::IBusFactory;
use super::dbus_server::service::IBusService;

pub struct FcpEngine {}

impl IBusEngine for FcpEngine {
    fn process_key_event(&mut self, keyval: u32, keycode: u32, state: u32) -> Result<bool, dbus::MethodErr> {
        println!("process_key_event: keyval={keyval} keycode={keycode} state={state}");
        Ok(true)
    }
    fn set_cursor_location(&mut self, x_: i32, y_: i32, w_: i32, h_: i32) -> Result<(), dbus::MethodErr> {
        println!("set_cursor_location: x={x_} y={y_} w={w_} h={h_}");
        Ok(())
    }
    fn process_hand_writing_event(&mut self, coordinates: Vec<f64>) -> Result<(), dbus::MethodErr> {
        println!("process_hand_writing_event");
        Ok(())
    }
    fn cancel_hand_writing(&mut self, n_strokes: u32) -> Result<(), dbus::MethodErr> {
        println!("cancel_hand_writing");
        Ok(())
    }
    fn set_capabilities(&mut self, caps: u32) -> Result<(), dbus::MethodErr> {
        println!("set_capabilities: caps={caps}");
        Ok(())
    }
    fn property_activate(&mut self, name: String, state: u32) -> Result<(), dbus::MethodErr> {
        println!("property_activate");
        Ok(())
    }
    fn property_show(&mut self, name: String) -> Result<(), dbus::MethodErr> {
        println!("property_show: name={name}");
        Ok(())
    }
    fn property_hide(&mut self, name: String) -> Result<(), dbus::MethodErr> {
        println!("property_hide: name={name}");
        Ok(())
    }
    fn candidate_clicked(&mut self, index: u32, button: u32, state: u32) -> Result<(), dbus::MethodErr> {
        println!("candidate_clicked: index={index} button={button} state={state}");
        Ok(())
    }
    fn focus_in(&mut self) -> Result<(), dbus::MethodErr> {
        println!("focus_in");
        Ok(())
    }
    fn focus_in_id(&mut self, object_path: String, client: String) -> Result<(), dbus::MethodErr> {
        println!("focus_in_id: object_path={object_path} client={client}");
        Ok(())
    }
    fn focus_out(&mut self) -> Result<(), dbus::MethodErr> {
        println!("focus_out");
        Ok(())
    }
    fn focus_out_id(&mut self, object_path: String) -> Result<(), dbus::MethodErr> {
        println!("focus_out_id: object_path={object_path}");
        Ok(())
    }
    fn reset(&mut self) -> Result<(), dbus::MethodErr> {
        println!("reset");
        Ok(())
    }
    fn enable(&mut self) -> Result<(), dbus::MethodErr> {
        println!("enable");
        Ok(())
    }
    fn disable(&mut self) -> Result<(), dbus::MethodErr> {
        println!("disable");
        Ok(())
    }
    fn page_up(&mut self) -> Result<(), dbus::MethodErr> {
        println!("page_up");
        Ok(())
    }
    fn page_down(&mut self) -> Result<(), dbus::MethodErr> {
        println!("page_down");
        Ok(())
    }
    fn cursor_up(&mut self) -> Result<(), dbus::MethodErr> {
        println!("cursor_up");
        Ok(())
    }
    fn cursor_down(&mut self) -> Result<(), dbus::MethodErr> {
        println!("cursor_down");
        Ok(())
    }
    fn set_surrounding_text(&mut self, text: arg::Variant<Box<dyn arg::RefArg + 'static>>, cursor_pos: u32, anchor_pos: u32) -> Result<(), dbus::MethodErr> {
        println!("set_surrounding_text: cursor_pos={cursor_pos}");
        Ok(())
    }
    fn panel_extension_received(&mut self, event: arg::Variant<Box<dyn arg::RefArg + 'static>>) -> Result<(), dbus::MethodErr> {
        println!("panel_extension_received");
        Ok(())
    }
    fn panel_extension_register_keys(&mut self, data: arg::Variant<Box<dyn arg::RefArg + 'static>>) -> Result<(), dbus::MethodErr> {
        println!("panel_extension_register_keys");
        Ok(())
    }
    fn set_content_type(&self, value: (u32, u32)) -> Result<(), dbus::MethodErr> {
        println!("set_content_type");
        Ok(())
    }
    fn focus_id(&self) -> Result<(bool), dbus::MethodErr> {
        println!("focus_id");
        Ok(true)
    }
    fn active_surrounding_text(&self) -> Result<(bool), dbus::MethodErr> {
        println!("active_surrounding_text");
        Ok(true)
    }
}

impl IBusFactory for FcpEngine {
    fn create_engine(&mut self, name: String) -> Result<dbus::Path<'static>, dbus::MethodErr> {
        let path = dbus::Path::from_slice("/org/freedesktop/IBus/Factory").expect("Failed to create DBus path.");
        Ok(path)
    }
}

impl IBusService for FcpEngine {
    fn destroy(&mut self) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
}