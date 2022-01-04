// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WpTransition")]
    pub struct Transition(Object<ffi::WpTransition, ffi::WpTransitionClass>) @implements gio::AsyncResult;

    match fn {
        type_ => || ffi::wp_transition_get_type(),
    }
}

impl Transition {
    //#[doc(alias = "wp_transition_new")]
    //pub fn new<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(type_: glib::types::Type, source_object: Option<&glib::Object>, cancellable: Option<&P>, callback: Q) -> Transition {
    //    unsafe { TODO: call ffi:wp_transition_new() }
    //}

    #[doc(alias = "wp_transition_new_closure")]
    pub fn new_closure<P: IsA<glib::Object>, Q: IsA<gio::Cancellable>>(type_: glib::types::Type, source_object: Option<&P>, cancellable: Option<&Q>, closure: Option<&glib::Closure>) -> Transition {
        unsafe {
            from_glib_none(ffi::wp_transition_new_closure(type_.into_glib(), source_object.map(|p| p.as_ref()).to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, closure.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_transition_finish")]
    pub fn finish<P: IsA<gio::AsyncResult>>(res: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::wp_transition_finish(res.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

pub const NONE_TRANSITION: Option<&Transition> = None;

pub trait TransitionExt: 'static {
    #[doc(alias = "wp_transition_advance")]
    fn advance(&self);

    #[doc(alias = "wp_transition_get_completed")]
    #[doc(alias = "get_completed")]
    fn is_completed(&self) -> bool;

    //#[doc(alias = "wp_transition_get_data")]
    //#[doc(alias = "get_data")]
    //fn data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[doc(alias = "wp_transition_get_source_object")]
    #[doc(alias = "get_source_object")]
    fn source_object(&self) -> Option<glib::Object>;

    //#[doc(alias = "wp_transition_get_source_tag")]
    //#[doc(alias = "get_source_tag")]
    //fn source_tag(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[doc(alias = "wp_transition_had_error")]
    fn had_error(&self) -> bool;

    //#[doc(alias = "wp_transition_is_tagged")]
    //fn is_tagged(&self, tag: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;

    //#[doc(alias = "wp_transition_set_data")]
    //fn set_data(&self, data: /*Unimplemented*/Option<Fundamental: Pointer>);

    //#[doc(alias = "wp_transition_set_source_tag")]
    //fn set_source_tag(&self, tag: /*Unimplemented*/Option<Fundamental: Pointer>);

    #[doc(alias = "completed")]
    fn connect_completed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Transition>> TransitionExt for O {
    fn advance(&self) {
        unsafe {
            ffi::wp_transition_advance(self.as_ref().to_glib_none().0);
        }
    }

    fn is_completed(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_transition_get_completed(self.as_ref().to_glib_none().0))
        }
    }

    //fn data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:wp_transition_get_data() }
    //}

    fn source_object(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::wp_transition_get_source_object(self.as_ref().to_glib_none().0))
        }
    }

    //fn source_tag(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:wp_transition_get_source_tag() }
    //}

    fn had_error(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_transition_had_error(self.as_ref().to_glib_none().0))
        }
    }

    //fn is_tagged(&self, tag: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:wp_transition_is_tagged() }
    //}

    //fn set_data(&self, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:wp_transition_set_data() }
    //}

    //fn set_source_tag(&self, tag: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:wp_transition_set_source_tag() }
    //}

    fn connect_completed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_completed_trampoline<P: IsA<Transition>, F: Fn(&P) + 'static>(this: *mut ffi::WpTransition, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Transition::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::completed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_completed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Transition")
    }
}
