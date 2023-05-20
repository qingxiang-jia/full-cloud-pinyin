//! # DBus interface proxies for: `org.freedesktop.IBus.Service`, `org.freedesktop.IBus.Factory`, `org.freedesktop.IBus.Service`, `org.freedesktop.IBus.Engine`, `org.freedesktop.IBus.Panel`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `interfaces.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.IBus.Factory", assume_defaults = true)]
trait Factory {
    /// CreateEngine method
    fn create_engine(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

#[dbus_proxy(interface = "org.freedesktop.IBus.Service", assume_defaults = true)]
trait Service {
    /// Destroy method
    fn destroy(&self) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.freedesktop.IBus.Engine", assume_defaults = true)]
trait Engine {
    /// CancelHandWriting method
    fn cancel_hand_writing(&self, n_strokes: u32) -> zbus::Result<()>;

    /// CandidateClicked method
    fn candidate_clicked(&self, index: u32, button: u32, state: u32) -> zbus::Result<()>;

    /// CursorDown method
    fn cursor_down(&self) -> zbus::Result<()>;

    /// CursorUp method
    fn cursor_up(&self) -> zbus::Result<()>;

    /// Disable method
    fn disable(&self) -> zbus::Result<()>;

    /// Enable method
    fn enable(&self) -> zbus::Result<()>;

    /// FocusIn method
    fn focus_in(&self) -> zbus::Result<()>;

    /// FocusIn method
    fn focus_in(&self) -> zbus::Result<()>;

    /// FocusInId method
    fn focus_in_id(&self, object_path: &str, client: &str) -> zbus::Result<()>;

    /// FocusOut method
    fn focus_out(&self) -> zbus::Result<()>;

    /// FocusOutId method
    fn focus_out_id(&self, object_path: &str) -> zbus::Result<()>;

    /// PageDown method
    fn page_down(&self) -> zbus::Result<()>;

    /// PageUp method
    fn page_up(&self) -> zbus::Result<()>;

    /// PanelExtensionReceived method
    fn panel_extension_received(&self, event: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// PanelExtensionRegisterKeys method
    fn panel_extension_register_keys(&self, data: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// ProcessHandWritingEvent method
    fn process_hand_writing_event(&self, coordinates: &[f64]) -> zbus::Result<()>;

    /// ProcessKeyEvent method
    fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> zbus::Result<bool>;

    /// PropertyActivate method
    fn property_activate(&self, name: &str, state: u32) -> zbus::Result<()>;

    /// PropertyHide method
    fn property_hide(&self, name: &str) -> zbus::Result<()>;

    /// PropertyShow method
    fn property_show(&self, name: &str) -> zbus::Result<()>;

    /// Reset method
    fn reset(&self) -> zbus::Result<()>;

    /// SetCapabilities method
    fn set_capabilities(&self, caps: u32) -> zbus::Result<()>;

    /// SetCursorLocation method
    fn set_cursor_location(&self, x: i32, y: i32, w: i32, h: i32) -> zbus::Result<()>;

    /// SetSurroundingText method
    fn set_surrounding_text(
        &self,
        text: &zbus::zvariant::Value<'_>,
        cursor_pos: u32,
        anchor_pos: u32,
    ) -> zbus::Result<()>;

    /// CommitText signal
    #[dbus_proxy(signal)]
    fn commit_text(&self, text: zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// ForwardKeyEvent signal
    #[dbus_proxy(signal)]
    fn forward_key_event(&self, keyval: u32, keycode: u32, state: u32) -> zbus::Result<()>;

    /// PanelExtension signal
    #[dbus_proxy(signal)]
    fn panel_extension(&self, data: zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// RegisterProperties signal
    #[dbus_proxy(signal)]
    fn register_properties(&self, props: zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// UpdateAuxiliaryText signal
    #[dbus_proxy(signal)]
    fn update_auxiliary_text(
        &self,
        text: zbus::zvariant::Value<'_>,
        visible: bool,
    ) -> zbus::Result<()>;

    /// UpdateLookupTable signal
    #[dbus_proxy(signal)]
    fn update_lookup_table(
        &self,
        table: zbus::zvariant::Value<'_>,
        visible: bool,
    ) -> zbus::Result<()>;

    /// UpdatePreeditText signal
    #[dbus_proxy(signal)]
    fn update_preedit_text(
        &self,
        text: zbus::zvariant::Value<'_>,
        cursor_pos: u32,
        visible: bool,
        mode: u32,
    ) -> zbus::Result<()>;

    /// UpdateProperty signal
    #[dbus_proxy(signal)]
    fn update_property(&self, prop: zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// ActiveSurroundingText property
    #[dbus_proxy(property)]
    fn active_surrounding_text(&self) -> zbus::Result<bool>;

    /// ContentType property
    #[dbus_proxy(property)]
    fn set_content_type(&self, value: &(u32, u32)) -> zbus::Result<()>;

    /// FocusId property
    #[dbus_proxy(property)]
    fn focus_id(&self) -> zbus::Result<bool>;
}

#[dbus_proxy(interface = "org.freedesktop.IBus.Panel", assume_defaults = true)]
trait Panel {
    /// CandidateClickedLookupTable method
    fn candidate_clicked_lookup_table(
        &self,
        index: u32,
        button: u32,
        state: u32,
    ) -> zbus::Result<()>;

    /// CommitTextReceived method
    fn commit_text_received(&self, text: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// ContentType method
    fn content_type(&self, purpose: u32, hints: u32) -> zbus::Result<()>;

    /// CursorDownLookupTable method
    fn cursor_down_lookup_table(&self) -> zbus::Result<()>;

    /// CursorUpLookupTable method
    fn cursor_up_lookup_table(&self) -> zbus::Result<()>;

    /// DestroyContext method
    fn destroy_context(&self, ic: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// FocusIn method
    fn focus_in(&self, ic: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// FocusOut method
    fn focus_out(&self, ic: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// HideAuxiliaryText method
    fn hide_auxiliary_text(&self) -> zbus::Result<()>;

    /// HideLanguageBar method
    fn hide_language_bar(&self) -> zbus::Result<()>;

    /// HideLookupTable method
    fn hide_lookup_table(&self) -> zbus::Result<()>;

    /// HidePreeditText method
    fn hide_preedit_text(&self) -> zbus::Result<()>;

    /// PageDownLookupTable method
    fn page_down_lookup_table(&self) -> zbus::Result<()>;

    /// PageUpLookupTable method
    fn page_up_lookup_table(&self) -> zbus::Result<()>;

    /// PanelExtensionReceived method
    fn panel_extension_received(&self, event: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// PanelExtensionRegisterKeys method
    fn panel_extension_register_keys(&self, data: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// ProcessKeyEvent method
    fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> zbus::Result<bool>;

    /// RegisterProperties method
    fn register_properties(&self, props: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// Reset method
    fn reset(&self) -> zbus::Result<()>;

    /// SetCursorLocation method
    fn set_cursor_location(&self, x: i32, y: i32, w: i32, h: i32) -> zbus::Result<()>;

    /// SetCursorLocationRelative method
    fn set_cursor_location_relative(&self, x: i32, y: i32, w: i32, h: i32) -> zbus::Result<()>;

    /// ShowAuxiliaryText method
    fn show_auxiliary_text(&self) -> zbus::Result<()>;

    /// ShowLanguageBar method
    fn show_language_bar(&self) -> zbus::Result<()>;

    /// ShowLookupTable method
    fn show_lookup_table(&self) -> zbus::Result<()>;

    /// ShowPreeditText method
    fn show_preedit_text(&self) -> zbus::Result<()>;

    /// StartSetup method
    fn start_setup(&self) -> zbus::Result<()>;

    /// StateChanged method
    fn state_changed(&self) -> zbus::Result<()>;

    /// UpdateAuxiliaryText method
    fn update_auxiliary_text(
        &self,
        text: &zbus::zvariant::Value<'_>,
        visible: bool,
    ) -> zbus::Result<()>;

    /// UpdateLookupTable method
    fn update_lookup_table(
        &self,
        table: &zbus::zvariant::Value<'_>,
        visible: bool,
    ) -> zbus::Result<()>;

    /// UpdatePreeditText method
    fn update_preedit_text(
        &self,
        text: &zbus::zvariant::Value<'_>,
        cursor_pos: u32,
        visible: bool,
    ) -> zbus::Result<()>;

    /// UpdateProperty method
    fn update_property(&self, prop: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// CandidateClicked signal
    #[dbus_proxy(signal)]
    fn candidate_clicked(&self, index: u32, button: u32, state: u32) -> zbus::Result<()>;

    /// CommitText signal
    #[dbus_proxy(signal)]
    fn commit_text(&self, text: zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// CursorDown signal
    #[dbus_proxy(signal)]
    fn cursor_down(&self) -> zbus::Result<()>;

    /// CursorUp signal
    #[dbus_proxy(signal)]
    fn cursor_up(&self) -> zbus::Result<()>;

    /// PageDown signal
    #[dbus_proxy(signal)]
    fn page_down(&self) -> zbus::Result<()>;

    /// PageUp signal
    #[dbus_proxy(signal)]
    fn page_up(&self) -> zbus::Result<()>;

    /// PanelExtension signal
    #[dbus_proxy(signal)]
    fn panel_extension(&self, event: zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    /// PropertyActivate signal
    #[dbus_proxy(signal)]
    fn property_activate(&self, prop_name: &str, prop_state: i32) -> zbus::Result<()>;

    /// PropertyHide signal
    #[dbus_proxy(signal)]
    fn property_hide(&self, prop_name: &str) -> zbus::Result<()>;

    /// PropertyShow signal
    #[dbus_proxy(signal)]
    fn property_show(&self, prop_name: &str) -> zbus::Result<()>;

    /// UpdateAuxiliaryTextReceived signal
    #[dbus_proxy(signal)]
    fn update_auxiliary_text_received(
        &self,
        text: zbus::zvariant::Value<'_>,
        visible: bool,
    ) -> zbus::Result<()>;

    /// UpdateLookupTableReceived signal
    #[dbus_proxy(signal)]
    fn update_lookup_table_received(
        &self,
        table: zbus::zvariant::Value<'_>,
        visible: bool,
    ) -> zbus::Result<()>;

    /// UpdatePreeditTextReceived signal
    #[dbus_proxy(signal)]
    fn update_preedit_text_received(
        &self,
        text: zbus::zvariant::Value<'_>,
        cursor_pos: u32,
        visible: bool,
    ) -> zbus::Result<()>;
}
