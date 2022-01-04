// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Core;
use crate::GlobalProxy;
use crate::Object;
use crate::PipewireObject;
use crate::Properties;
use crate::Proxy;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WpDevice")]
    pub struct Device(Object<ffi::WpDevice, ffi::WpDeviceClass>) @extends GlobalProxy, Proxy, Object, @implements PipewireObject;

    match fn {
        type_ => || ffi::wp_device_get_type(),
    }
}

impl Device {
    #[doc(alias = "wp_device_new_from_factory")]
    #[doc(alias = "new_from_factory")]
    pub fn from_factory(core: &Core, factory_name: &str, properties: Option<&Properties>) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::wp_device_new_from_factory(core.to_glib_none().0, factory_name.to_glib_none().0, properties.to_glib_full()))
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Device")
    }
}
