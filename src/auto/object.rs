// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Core;
use crate::ObjectFeatures;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WpObject")]
    pub struct Object(Object<ffi::WpObject, ffi::WpObjectClass>);

    match fn {
        type_ => || ffi::wp_object_get_type(),
    }
}

pub const NONE_OBJECT: Option<&Object> = None;

pub trait ObjectExt: 'static {
    #[doc(alias = "wp_object_activate")]
    fn activate<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, features: ObjectFeatures, cancellable: Option<&P>, callback: Q);

    
    fn activate_future(&self, features: ObjectFeatures) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "wp_object_activate_closure")]
    fn activate_closure<P: IsA<gio::Cancellable>>(&self, features: ObjectFeatures, cancellable: Option<&P>, closure: &glib::Closure);

    #[doc(alias = "wp_object_deactivate")]
    fn deactivate(&self, features: ObjectFeatures);

    #[doc(alias = "wp_object_get_active_features")]
    #[doc(alias = "get_active_features")]
    fn active_features(&self) -> ObjectFeatures;

    #[doc(alias = "wp_object_get_core")]
    #[doc(alias = "get_core")]
    fn core(&self) -> Option<Core>;

    #[doc(alias = "wp_object_get_supported_features")]
    #[doc(alias = "get_supported_features")]
    fn supported_features(&self) -> ObjectFeatures;

    #[doc(alias = "wp_object_update_features")]
    fn update_features(&self, activated: ObjectFeatures, deactivated: ObjectFeatures);

    #[doc(alias = "active-features")]
    fn connect_active_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "supported-features")]
    fn connect_supported_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Object>> ObjectExt for O {
    fn activate<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, features: ObjectFeatures, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn activate_trampoline<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::wp_object_activate_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = activate_trampoline::<Q>;
        unsafe {
            ffi::wp_object_activate(self.as_ref().to_glib_none().0, features, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn activate_future(&self, features: ObjectFeatures) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.activate(
                features,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    fn activate_closure<P: IsA<gio::Cancellable>>(&self, features: ObjectFeatures, cancellable: Option<&P>, closure: &glib::Closure) {
        unsafe {
            ffi::wp_object_activate_closure(self.as_ref().to_glib_none().0, features, cancellable.map(|p| p.as_ref()).to_glib_none().0, closure.to_glib_full());
        }
    }

    fn deactivate(&self, features: ObjectFeatures) {
        unsafe {
            ffi::wp_object_deactivate(self.as_ref().to_glib_none().0, features);
        }
    }

    fn active_features(&self) -> ObjectFeatures {
        unsafe {
            ffi::wp_object_get_active_features(self.as_ref().to_glib_none().0)
        }
    }

    fn core(&self) -> Option<Core> {
        unsafe {
            from_glib_full(ffi::wp_object_get_core(self.as_ref().to_glib_none().0))
        }
    }

    fn supported_features(&self) -> ObjectFeatures {
        unsafe {
            ffi::wp_object_get_supported_features(self.as_ref().to_glib_none().0)
        }
    }

    fn update_features(&self, activated: ObjectFeatures, deactivated: ObjectFeatures) {
        unsafe {
            ffi::wp_object_update_features(self.as_ref().to_glib_none().0, activated, deactivated);
        }
    }

    fn connect_active_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_features_trampoline<P: IsA<Object>, F: Fn(&P) + 'static>(this: *mut ffi::WpObject, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active-features\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_active_features_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_supported_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_supported_features_trampoline<P: IsA<Object>, F: Fn(&P) + 'static>(this: *mut ffi::WpObject, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::supported-features\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_supported_features_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Object")
    }
}
