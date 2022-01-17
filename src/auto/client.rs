// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::GlobalProxy;
use crate::Object;
use crate::PipewireObject;
use crate::Proxy;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "WpClient")]
    pub struct Client(Object<ffi::WpClient, ffi::WpClientClass>) @extends GlobalProxy, Proxy, Object, @implements PipewireObject;

    match fn {
        type_ => || ffi::wp_client_get_type(),
    }
}

impl Client {
    #[doc(alias = "wp_client_send_error")]
    pub fn send_error(&self, id: u32, res: i32, message: &str) {
        unsafe {
            ffi::wp_client_send_error(self.to_glib_none().0, id, res, message.to_glib_none().0);
        }
    }

    //#[doc(alias = "wp_client_update_permissions")]
    //pub fn update_permissions(&self, n_perm: u32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:wp_client_update_permissions() }
    //}

    #[doc(alias = "wp_client_update_permissions_array")]
    pub fn update_permissions_array(&self, permissions: &[u64]) {
        let n_perm = permissions.len() as u32;
        unsafe {
            ffi::wp_client_update_permissions_array(self.to_glib_none().0, n_perm, permissions.to_glib_none().0);
        }
    }
}
