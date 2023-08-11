// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Core,DBusState,Object};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,mem::transmute};

glib::wrapper! {
    #[doc(alias = "WpDbus")]
    pub struct Dbus(Object<ffi::WpDbus, ffi::WpDbusClass>) @extends Object;

    match fn {
        type_ => || ffi::wp_dbus_get_type(),
    }
}

impl Dbus {
    #[doc(alias = "wp_dbus_get_bus_type")]
    #[doc(alias = "get_bus_type")]
    pub fn bus_type(&self) -> gio::BusType {
        unsafe {
            from_glib(ffi::wp_dbus_get_bus_type(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_dbus_get_connection")]
    #[doc(alias = "get_connection")]
    pub fn connection(&self) -> Option<gio::DBusConnection> {
        unsafe {
            from_glib_none(ffi::wp_dbus_get_connection(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_dbus_get_state")]
    #[doc(alias = "get_state")]
    pub fn state(&self) -> DBusState {
        unsafe {
            from_glib(ffi::wp_dbus_get_state(self.to_glib_none().0))
        }
    }

    #[doc(alias = "bus-type")]
    pub fn get_property_bus_type(&self) -> gio::BusType {
        ObjectExt::property(self, "bus-type")
    }

    pub fn get_property_state(&self) -> DBusState {
        ObjectExt::property(self, "state")
    }

    #[doc(alias = "wp_dbus_get_instance")]
    #[doc(alias = "get_instance")]
    pub fn instance(core: &Core, bus_type: gio::BusType) -> Option<Dbus> {
        unsafe {
            from_glib_none(ffi::wp_dbus_get_instance(core.to_glib_none().0, bus_type.into_glib()))
        }
    }

    #[doc(alias = "state")]
    pub fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<F: Fn(&Dbus) + 'static>(this: *mut ffi::WpDbus, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_state_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
