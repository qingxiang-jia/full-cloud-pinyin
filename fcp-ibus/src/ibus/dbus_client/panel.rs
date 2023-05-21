// This code was autogenerated with `dbus-codegen-rust -c blocking --file ./interfaces_you_call.xml -o you_call.rs`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopIBusPanel {
    fn update_preedit_text(&self, text: arg::Variant<Box<dyn arg::RefArg>>, cursor_pos: u32, visible: bool) -> Result<(), dbus::Error>;
    fn show_preedit_text(&self) -> Result<(), dbus::Error>;
    fn hide_preedit_text(&self) -> Result<(), dbus::Error>;
    fn update_auxiliary_text(&self, text: arg::Variant<Box<dyn arg::RefArg>>, visible: bool) -> Result<(), dbus::Error>;
    fn show_auxiliary_text(&self) -> Result<(), dbus::Error>;
    fn hide_auxiliary_text(&self) -> Result<(), dbus::Error>;
    fn update_lookup_table(&self, table: arg::Variant<Box<dyn arg::RefArg>>, visible: bool) -> Result<(), dbus::Error>;
    fn show_lookup_table(&self) -> Result<(), dbus::Error>;
    fn hide_lookup_table(&self) -> Result<(), dbus::Error>;
    fn cursor_up_lookup_table(&self) -> Result<(), dbus::Error>;
    fn cursor_down_lookup_table(&self) -> Result<(), dbus::Error>;
    fn page_up_lookup_table(&self) -> Result<(), dbus::Error>;
    fn page_down_lookup_table(&self) -> Result<(), dbus::Error>;
    fn candidate_clicked_lookup_table(&self, index: u32, button: u32, state: u32) -> Result<(), dbus::Error>;
    fn register_properties(&self, props: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
    fn update_property(&self, prop: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
    fn focus_in(&self, ic: dbus::Path) -> Result<(), dbus::Error>;
    fn focus_out(&self, ic: dbus::Path) -> Result<(), dbus::Error>;
    fn destroy_context(&self, ic: dbus::Path) -> Result<(), dbus::Error>;
    fn set_cursor_location(&self, x_: i32, y_: i32, w_: i32, h_: i32) -> Result<(), dbus::Error>;
    fn set_cursor_location_relative(&self, x_: i32, y_: i32, w_: i32, h_: i32) -> Result<(), dbus::Error>;
    fn reset(&self) -> Result<(), dbus::Error>;
    fn start_setup(&self) -> Result<(), dbus::Error>;
    fn state_changed(&self) -> Result<(), dbus::Error>;
    fn hide_language_bar(&self) -> Result<(), dbus::Error>;
    fn show_language_bar(&self) -> Result<(), dbus::Error>;
    fn content_type(&self, purpose: u32, hints: u32) -> Result<(), dbus::Error>;
    fn panel_extension_received(&self, event: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
    fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> Result<bool, dbus::Error>;
    fn commit_text_received(&self, text: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
    fn panel_extension_register_keys(&self, data: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelCursorUp {
}

impl arg::AppendAll for OrgFreedesktopIBusPanelCursorUp {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelCursorUp {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelCursorUp {
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelCursorUp {
    const NAME: &'static str = "CursorUp";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelCursorDown {
}

impl arg::AppendAll for OrgFreedesktopIBusPanelCursorDown {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelCursorDown {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelCursorDown {
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelCursorDown {
    const NAME: &'static str = "CursorDown";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelPageUp {
}

impl arg::AppendAll for OrgFreedesktopIBusPanelPageUp {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelPageUp {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelPageUp {
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelPageUp {
    const NAME: &'static str = "PageUp";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelPageDown {
}

impl arg::AppendAll for OrgFreedesktopIBusPanelPageDown {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelPageDown {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelPageDown {
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelPageDown {
    const NAME: &'static str = "PageDown";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelPropertyActivate {
    pub prop_name: String,
    pub prop_state: i32,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelPropertyActivate {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.prop_name, i);
        arg::RefArg::append(&self.prop_state, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelPropertyActivate {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelPropertyActivate {
            prop_name: i.read()?,
            prop_state: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelPropertyActivate {
    const NAME: &'static str = "PropertyActivate";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelPropertyShow {
    pub prop_name: String,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelPropertyShow {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.prop_name, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelPropertyShow {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelPropertyShow {
            prop_name: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelPropertyShow {
    const NAME: &'static str = "PropertyShow";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelPropertyHide {
    pub prop_name: String,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelPropertyHide {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.prop_name, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelPropertyHide {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelPropertyHide {
            prop_name: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelPropertyHide {
    const NAME: &'static str = "PropertyHide";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelCandidateClicked {
    pub index: u32,
    pub button: u32,
    pub state: u32,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelCandidateClicked {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.index, i);
        arg::RefArg::append(&self.button, i);
        arg::RefArg::append(&self.state, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelCandidateClicked {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelCandidateClicked {
            index: i.read()?,
            button: i.read()?,
            state: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelCandidateClicked {
    const NAME: &'static str = "CandidateClicked";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelCommitText {
    pub text: arg::Variant<Box<dyn arg::RefArg + 'static>>,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelCommitText {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.text, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelCommitText {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelCommitText {
            text: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelCommitText {
    const NAME: &'static str = "CommitText";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelPanelExtension {
    pub event: arg::Variant<Box<dyn arg::RefArg + 'static>>,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelPanelExtension {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.event, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelPanelExtension {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelPanelExtension {
            event: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelPanelExtension {
    const NAME: &'static str = "PanelExtension";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelUpdatePreeditTextReceived {
    pub text: arg::Variant<Box<dyn arg::RefArg + 'static>>,
    pub cursor_pos: u32,
    pub visible: bool,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelUpdatePreeditTextReceived {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.text, i);
        arg::RefArg::append(&self.cursor_pos, i);
        arg::RefArg::append(&self.visible, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelUpdatePreeditTextReceived {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelUpdatePreeditTextReceived {
            text: i.read()?,
            cursor_pos: i.read()?,
            visible: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelUpdatePreeditTextReceived {
    const NAME: &'static str = "UpdatePreeditTextReceived";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelUpdateAuxiliaryTextReceived {
    pub text: arg::Variant<Box<dyn arg::RefArg + 'static>>,
    pub visible: bool,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelUpdateAuxiliaryTextReceived {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.text, i);
        arg::RefArg::append(&self.visible, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelUpdateAuxiliaryTextReceived {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelUpdateAuxiliaryTextReceived {
            text: i.read()?,
            visible: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelUpdateAuxiliaryTextReceived {
    const NAME: &'static str = "UpdateAuxiliaryTextReceived";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

#[derive(Debug)]
pub struct OrgFreedesktopIBusPanelUpdateLookupTableReceived {
    pub table: arg::Variant<Box<dyn arg::RefArg + 'static>>,
    pub visible: bool,
}

impl arg::AppendAll for OrgFreedesktopIBusPanelUpdateLookupTableReceived {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.table, i);
        arg::RefArg::append(&self.visible, i);
    }
}

impl arg::ReadAll for OrgFreedesktopIBusPanelUpdateLookupTableReceived {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopIBusPanelUpdateLookupTableReceived {
            table: i.read()?,
            visible: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopIBusPanelUpdateLookupTableReceived {
    const NAME: &'static str = "UpdateLookupTableReceived";
    const INTERFACE: &'static str = "org.freedesktop.IBus.Panel";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopIBusPanel for blocking::Proxy<'a, C> {

    fn update_preedit_text(&self, text: arg::Variant<Box<dyn arg::RefArg>>, cursor_pos: u32, visible: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "UpdatePreeditText", (text, cursor_pos, visible, ))
    }

    fn show_preedit_text(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "ShowPreeditText", ())
    }

    fn hide_preedit_text(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "HidePreeditText", ())
    }

    fn update_auxiliary_text(&self, text: arg::Variant<Box<dyn arg::RefArg>>, visible: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "UpdateAuxiliaryText", (text, visible, ))
    }

    fn show_auxiliary_text(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "ShowAuxiliaryText", ())
    }

    fn hide_auxiliary_text(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "HideAuxiliaryText", ())
    }

    fn update_lookup_table(&self, table: arg::Variant<Box<dyn arg::RefArg>>, visible: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "UpdateLookupTable", (table, visible, ))
    }

    fn show_lookup_table(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "ShowLookupTable", ())
    }

    fn hide_lookup_table(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "HideLookupTable", ())
    }

    fn cursor_up_lookup_table(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "CursorUpLookupTable", ())
    }

    fn cursor_down_lookup_table(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "CursorDownLookupTable", ())
    }

    fn page_up_lookup_table(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "PageUpLookupTable", ())
    }

    fn page_down_lookup_table(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "PageDownLookupTable", ())
    }

    fn candidate_clicked_lookup_table(&self, index: u32, button: u32, state: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "CandidateClickedLookupTable", (index, button, state, ))
    }

    fn register_properties(&self, props: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "RegisterProperties", (props, ))
    }

    fn update_property(&self, prop: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "UpdateProperty", (prop, ))
    }

    fn focus_in(&self, ic: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "FocusIn", (ic, ))
    }

    fn focus_out(&self, ic: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "FocusOut", (ic, ))
    }

    fn destroy_context(&self, ic: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "DestroyContext", (ic, ))
    }

    fn set_cursor_location(&self, x_: i32, y_: i32, w_: i32, h_: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "SetCursorLocation", (x_, y_, w_, h_, ))
    }

    fn set_cursor_location_relative(&self, x_: i32, y_: i32, w_: i32, h_: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "SetCursorLocationRelative", (x_, y_, w_, h_, ))
    }

    fn reset(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "Reset", ())
    }

    fn start_setup(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "StartSetup", ())
    }

    fn state_changed(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "StateChanged", ())
    }

    fn hide_language_bar(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "HideLanguageBar", ())
    }

    fn show_language_bar(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "ShowLanguageBar", ())
    }

    fn content_type(&self, purpose: u32, hints: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "ContentType", (purpose, hints, ))
    }

    fn panel_extension_received(&self, event: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "PanelExtensionReceived", (event, ))
    }

    fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> Result<bool, dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "ProcessKeyEvent", (keyval, keycode, state, ))
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn commit_text_received(&self, text: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "CommitTextReceived", (text, ))
    }

    fn panel_extension_register_keys(&self, data: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.IBus.Panel", "PanelExtensionRegisterKeys", (data, ))
    }
}