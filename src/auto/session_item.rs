// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Core;
use crate::Object;
use crate::Properties;
use crate::Proxy;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WpSessionItem")]
    pub struct SessionItem(Object<ffi::WpSessionItem, ffi::WpSessionItemClass>) @extends Object;

    match fn {
        type_ => || ffi::wp_session_item_get_type(),
    }
}

impl SessionItem {
    #[doc(alias = "wp_session_item_handle_proxy_destroyed")]
    pub fn handle_proxy_destroyed<P: IsA<Proxy>, Q: IsA<SessionItem>>(proxy: &P, item: &Q) {
        unsafe {
            ffi::wp_session_item_handle_proxy_destroyed(proxy.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "wp_session_item_make")]
    pub fn make(core: &Core, factory_name: &str) -> Option<SessionItem> {
        unsafe {
            from_glib_full(ffi::wp_session_item_make(core.to_glib_none().0, factory_name.to_glib_none().0))
        }
    }
}

pub const NONE_SESSION_ITEM: Option<&SessionItem> = None;

pub trait SessionItemExt: 'static {
    #[doc(alias = "wp_session_item_configure")]
    fn configure(&self, props: &Properties) -> bool;

    #[doc(alias = "wp_session_item_get_associated_proxy")]
    #[doc(alias = "get_associated_proxy")]
    fn associated_proxy(&self, proxy_type: glib::types::Type) -> Option<Proxy>;

    #[doc(alias = "wp_session_item_get_associated_proxy_id")]
    #[doc(alias = "get_associated_proxy_id")]
    fn associated_proxy_id(&self, proxy_type: glib::types::Type) -> u32;

    #[doc(alias = "wp_session_item_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> u32;

    #[doc(alias = "wp_session_item_get_properties")]
    #[doc(alias = "get_properties")]
    fn properties(&self) -> Option<Properties>;

    #[doc(alias = "wp_session_item_get_property")]
    #[doc(alias = "get_property")]
    fn property(&self, key: &str) -> Option<glib::GString>;

    #[doc(alias = "wp_session_item_is_configured")]
    fn is_configured(&self) -> bool;

    #[doc(alias = "wp_session_item_register")]
    fn register(&self);

    #[doc(alias = "wp_session_item_remove")]
    fn remove(&self);

    #[doc(alias = "wp_session_item_reset")]
    fn reset(&self);

    #[doc(alias = "wp_session_item_set_properties")]
    fn set_properties(&self, props: &Properties);

    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "properties")]
    fn connect_properties_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SessionItem>> SessionItemExt for O {
    fn configure(&self, props: &Properties) -> bool {
        unsafe {
            from_glib(ffi::wp_session_item_configure(self.as_ref().to_glib_none().0, props.to_glib_full()))
        }
    }

    fn associated_proxy(&self, proxy_type: glib::types::Type) -> Option<Proxy> {
        unsafe {
            from_glib_full(ffi::wp_session_item_get_associated_proxy(self.as_ref().to_glib_none().0, proxy_type.into_glib()))
        }
    }

    fn associated_proxy_id(&self, proxy_type: glib::types::Type) -> u32 {
        unsafe {
            ffi::wp_session_item_get_associated_proxy_id(self.as_ref().to_glib_none().0, proxy_type.into_glib())
        }
    }

    fn id(&self) -> u32 {
        unsafe {
            ffi::wp_session_item_get_id(self.as_ref().to_glib_none().0)
        }
    }

    fn properties(&self) -> Option<Properties> {
        unsafe {
            from_glib_full(ffi::wp_session_item_get_properties(self.as_ref().to_glib_none().0))
        }
    }

    fn property(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_session_item_get_property(self.as_ref().to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn is_configured(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_session_item_is_configured(self.as_ref().to_glib_none().0))
        }
    }

    fn register(&self) {
        unsafe {
            ffi::wp_session_item_register(self.as_ref().to_glib_full());
        }
    }

    fn remove(&self) {
        unsafe {
            ffi::wp_session_item_remove(self.as_ref().to_glib_none().0);
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::wp_session_item_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_properties(&self, props: &Properties) {
        unsafe {
            ffi::wp_session_item_set_properties(self.as_ref().to_glib_none().0, props.to_glib_full());
        }
    }

    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<SessionItem>, F: Fn(&P) + 'static>(this: *mut ffi::WpSessionItem, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(SessionItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_properties_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_properties_trampoline<P: IsA<SessionItem>, F: Fn(&P) + 'static>(this: *mut ffi::WpSessionItem, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(SessionItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::properties\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_properties_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for SessionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SessionItem")
    }
}
