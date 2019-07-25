// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use AppLaunchContext;
use Clipboard;
use Device;
use Event;
use Keymap;
use Monitor;
use Seat;
use Surface;

glib_wrapper! {
    pub struct Display(Object<gdk_sys::GdkDisplay, DisplayClass>);

    match fn {
        get_type => || gdk_sys::gdk_display_get_type(),
    }
}

impl Display {
    pub fn beep(&self) {
        unsafe {
            gdk_sys::gdk_display_beep(self.to_glib_none().0);
        }
    }

    pub fn close(&self) {
        unsafe {
            gdk_sys::gdk_display_close(self.to_glib_none().0);
        }
    }

    pub fn device_is_grabbed(&self, device: &Device) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_display_device_is_grabbed(
                self.to_glib_none().0,
                device.to_glib_none().0,
            ))
        }
    }

    pub fn flush(&self) {
        unsafe {
            gdk_sys::gdk_display_flush(self.to_glib_none().0);
        }
    }

    pub fn get_app_launch_context(&self) -> AppLaunchContext {
        unsafe {
            from_glib_full(gdk_sys::gdk_display_get_app_launch_context(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_clipboard(&self) -> Clipboard {
        unsafe { from_glib_none(gdk_sys::gdk_display_get_clipboard(self.to_glib_none().0)) }
    }

    pub fn get_default_group(&self) -> Surface {
        unsafe {
            from_glib_none(gdk_sys::gdk_display_get_default_group(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_default_seat(&self) -> Seat {
        unsafe { from_glib_none(gdk_sys::gdk_display_get_default_seat(self.to_glib_none().0)) }
    }

    pub fn get_event(&self) -> Option<Event> {
        unsafe { from_glib_full(gdk_sys::gdk_display_get_event(self.to_glib_none().0)) }
    }

    pub fn get_keymap(&self) -> Keymap {
        unsafe { from_glib_none(gdk_sys::gdk_display_get_keymap(self.to_glib_none().0)) }
    }

    pub fn get_monitor(&self, monitor_num: i32) -> Option<Monitor> {
        unsafe {
            from_glib_none(gdk_sys::gdk_display_get_monitor(
                self.to_glib_none().0,
                monitor_num,
            ))
        }
    }

    pub fn get_monitor_at_point(&self, x: i32, y: i32) -> Option<Monitor> {
        unsafe {
            from_glib_none(gdk_sys::gdk_display_get_monitor_at_point(
                self.to_glib_none().0,
                x,
                y,
            ))
        }
    }

    pub fn get_monitor_at_surface<P: IsA<Surface>>(&self, surface: &P) -> Option<Monitor> {
        unsafe {
            from_glib_none(gdk_sys::gdk_display_get_monitor_at_surface(
                self.to_glib_none().0,
                surface.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn get_n_monitors(&self) -> i32 {
        unsafe { gdk_sys::gdk_display_get_n_monitors(self.to_glib_none().0) }
    }

    pub fn get_name(&self) -> GString {
        unsafe { from_glib_none(gdk_sys::gdk_display_get_name(self.to_glib_none().0)) }
    }

    pub fn get_primary_clipboard(&self) -> Clipboard {
        unsafe {
            from_glib_none(gdk_sys::gdk_display_get_primary_clipboard(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_primary_monitor(&self) -> Monitor {
        unsafe {
            from_glib_none(gdk_sys::gdk_display_get_primary_monitor(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_setting(&self, name: &str, value: &mut glib::Value) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_display_get_setting(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none_mut().0,
            ))
        }
    }

    pub fn get_startup_notification_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gdk_sys::gdk_display_get_startup_notification_id(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn has_pending(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_display_has_pending(self.to_glib_none().0)) }
    }

    pub fn is_closed(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_display_is_closed(self.to_glib_none().0)) }
    }

    pub fn is_composited(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_display_is_composited(self.to_glib_none().0)) }
    }

    pub fn is_rgba(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_display_is_rgba(self.to_glib_none().0)) }
    }

    pub fn list_seats(&self) -> Vec<Seat> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_display_list_seats(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn notify_startup_complete(&self, startup_id: &str) {
        unsafe {
            gdk_sys::gdk_display_notify_startup_complete(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    pub fn peek_event(&self) -> Option<Event> {
        unsafe { from_glib_full(gdk_sys::gdk_display_peek_event(self.to_glib_none().0)) }
    }

    pub fn put_event(&self, event: &Event) {
        unsafe {
            gdk_sys::gdk_display_put_event(self.to_glib_none().0, event.to_glib_none().0);
        }
    }

    pub fn supports_input_shapes(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_display_supports_input_shapes(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn supports_shapes(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_display_supports_shapes(self.to_glib_none().0)) }
    }

    pub fn sync(&self) {
        unsafe {
            gdk_sys::gdk_display_sync(self.to_glib_none().0);
        }
    }

    pub fn get_property_composited(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"composited\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    pub fn get_property_rgba(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"rgba\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    pub fn get_default() -> Option<Display> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gdk_sys::gdk_display_get_default()) }
    }

    pub fn open(display_name: &str) -> Option<Display> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gdk_sys::gdk_display_open(display_name.to_glib_none().0)) }
    }

    pub fn connect_closed<F: Fn(&Display, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<F: Fn(&Display, bool) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            is_error: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(is_error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(transmute(closed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_monitor_added<F: Fn(&Display, &Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn monitor_added_trampoline<F: Fn(&Display, &Monitor) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            monitor: *mut gdk_sys::GdkMonitor,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"monitor-added\0".as_ptr() as *const _,
                Some(transmute(monitor_added_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_monitor_removed<F: Fn(&Display, &Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn monitor_removed_trampoline<F: Fn(&Display, &Monitor) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            monitor: *mut gdk_sys::GdkMonitor,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"monitor-removed\0".as_ptr() as *const _,
                Some(transmute(monitor_removed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_opened<F: Fn(&Display) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn opened_trampoline<F: Fn(&Display) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"opened\0".as_ptr() as *const _,
                Some(transmute(opened_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_seat_added<F: Fn(&Display, &Seat) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn seat_added_trampoline<F: Fn(&Display, &Seat) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            seat: *mut gdk_sys::GdkSeat,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(seat))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"seat-added\0".as_ptr() as *const _,
                Some(transmute(seat_added_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_seat_removed<F: Fn(&Display, &Seat) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn seat_removed_trampoline<F: Fn(&Display, &Seat) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            seat: *mut gdk_sys::GdkSeat,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(seat))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"seat-removed\0".as_ptr() as *const _,
                Some(transmute(seat_removed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_setting_changed<F: Fn(&Display, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn setting_changed_trampoline<F: Fn(&Display, &str) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            setting: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &GString::from_glib_borrow(setting))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"setting-changed\0".as_ptr() as *const _,
                Some(transmute(setting_changed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_composited_notify<F: Fn(&Display) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_composited_trampoline<F: Fn(&Display) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::composited\0".as_ptr() as *const _,
                Some(transmute(notify_composited_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_rgba_notify<F: Fn(&Display) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rgba_trampoline<F: Fn(&Display) + 'static>(
            this: *mut gdk_sys::GdkDisplay,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rgba\0".as_ptr() as *const _,
                Some(transmute(notify_rgba_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display")
    }
}
