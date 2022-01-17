// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;

bitflags! {
    #[doc(alias = "WpInitFlags")]
    pub struct InitFlags: u32 {
        #[doc(alias = "WP_INIT_PIPEWIRE")]
        const PIPEWIRE = ffi::WP_INIT_PIPEWIRE as u32;
        #[doc(alias = "WP_INIT_SPA_TYPES")]
        const SPA_TYPES = ffi::WP_INIT_SPA_TYPES as u32;
        #[doc(alias = "WP_INIT_SET_PW_LOG")]
        const SET_PW_LOG = ffi::WP_INIT_SET_PW_LOG as u32;
        #[doc(alias = "WP_INIT_SET_GLIB_LOG")]
        const SET_GLIB_LOG = ffi::WP_INIT_SET_GLIB_LOG as u32;
        #[doc(alias = "WP_INIT_ALL")]
        const ALL = ffi::WP_INIT_ALL as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for InitFlags {
    type GlibType = ffi::WpInitFlags;

    fn into_glib(self) -> ffi::WpInitFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpInitFlags> for InitFlags {
    unsafe fn from_glib(value: ffi::WpInitFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InitFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_init_flags_get_type()) }
    }
}

impl glib::value::ValueType for InitFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InitFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InitFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpInterestMatch")]
    pub struct InterestMatch: u32 {
        #[doc(alias = "WP_INTEREST_MATCH_NONE")]
        const NONE = ffi::WP_INTEREST_MATCH_NONE as u32;
        #[doc(alias = "WP_INTEREST_MATCH_GTYPE")]
        const GTYPE = ffi::WP_INTEREST_MATCH_GTYPE as u32;
        #[doc(alias = "WP_INTEREST_MATCH_PW_GLOBAL_PROPERTIES")]
        const PW_GLOBAL_PROPERTIES = ffi::WP_INTEREST_MATCH_PW_GLOBAL_PROPERTIES as u32;
        #[doc(alias = "WP_INTEREST_MATCH_PW_PROPERTIES")]
        const PW_PROPERTIES = ffi::WP_INTEREST_MATCH_PW_PROPERTIES as u32;
        #[doc(alias = "WP_INTEREST_MATCH_G_PROPERTIES")]
        const G_PROPERTIES = ffi::WP_INTEREST_MATCH_G_PROPERTIES as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for InterestMatch {
    type GlibType = ffi::WpInterestMatch;

    fn into_glib(self) -> ffi::WpInterestMatch {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpInterestMatch> for InterestMatch {
    unsafe fn from_glib(value: ffi::WpInterestMatch) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InterestMatch {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_interest_match_get_type()) }
    }
}

impl glib::value::ValueType for InterestMatch {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InterestMatch {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InterestMatch {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpInterestMatchFlags")]
    pub struct InterestMatchFlags: u32 {
        #[doc(alias = "WP_INTEREST_MATCH_FLAGS_NONE")]
        const NONE = ffi::WP_INTEREST_MATCH_FLAGS_NONE as u32;
        #[doc(alias = "WP_INTEREST_MATCH_FLAGS_CHECK_ALL")]
        const CHECK_ALL = ffi::WP_INTEREST_MATCH_FLAGS_CHECK_ALL as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for InterestMatchFlags {
    type GlibType = ffi::WpInterestMatchFlags;

    fn into_glib(self) -> ffi::WpInterestMatchFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpInterestMatchFlags> for InterestMatchFlags {
    unsafe fn from_glib(value: ffi::WpInterestMatchFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InterestMatchFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_interest_match_flags_get_type()) }
    }
}

impl glib::value::ValueType for InterestMatchFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InterestMatchFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InterestMatchFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpLookupDirs")]
    pub struct LookupDirs: u32 {
        #[doc(alias = "WP_LOOKUP_DIR_ENV_CONFIG")]
        const ENV_CONFIG = ffi::WP_LOOKUP_DIR_ENV_CONFIG as u32;
        #[doc(alias = "WP_LOOKUP_DIR_ENV_DATA")]
        const ENV_DATA = ffi::WP_LOOKUP_DIR_ENV_DATA as u32;
        #[doc(alias = "WP_LOOKUP_DIR_XDG_CONFIG_HOME")]
        const XDG_CONFIG_HOME = ffi::WP_LOOKUP_DIR_XDG_CONFIG_HOME as u32;
        #[doc(alias = "WP_LOOKUP_DIR_ETC")]
        const ETC = ffi::WP_LOOKUP_DIR_ETC as u32;
        #[doc(alias = "WP_LOOKUP_DIR_PREFIX_SHARE")]
        const PREFIX_SHARE = ffi::WP_LOOKUP_DIR_PREFIX_SHARE as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for LookupDirs {
    type GlibType = ffi::WpLookupDirs;

    fn into_glib(self) -> ffi::WpLookupDirs {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpLookupDirs> for LookupDirs {
    unsafe fn from_glib(value: ffi::WpLookupDirs) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for LookupDirs {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_lookup_dirs_get_type()) }
    }
}

impl glib::value::ValueType for LookupDirs {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for LookupDirs {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for LookupDirs {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpMetadataFeatures")]
    pub struct MetadataFeatures: u32 {
        #[doc(alias = "WP_METADATA_FEATURE_DATA")]
        const DATA = ffi::WP_METADATA_FEATURE_DATA as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for MetadataFeatures {
    type GlibType = ffi::WpMetadataFeatures;

    fn into_glib(self) -> ffi::WpMetadataFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpMetadataFeatures> for MetadataFeatures {
    unsafe fn from_glib(value: ffi::WpMetadataFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for MetadataFeatures {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_metadata_features_get_type()) }
    }
}

impl glib::value::ValueType for MetadataFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for MetadataFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for MetadataFeatures {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpNodeFeatures")]
    pub struct NodeFeatures: u32 {
        #[doc(alias = "WP_NODE_FEATURE_PORTS")]
        const PORTS = ffi::WP_NODE_FEATURE_PORTS as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for NodeFeatures {
    type GlibType = ffi::WpNodeFeatures;

    fn into_glib(self) -> ffi::WpNodeFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpNodeFeatures> for NodeFeatures {
    unsafe fn from_glib(value: ffi::WpNodeFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for NodeFeatures {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_node_features_get_type()) }
    }
}

impl glib::value::ValueType for NodeFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for NodeFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for NodeFeatures {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpPluginFeatures")]
    pub struct PluginFeatures: u32 {
        #[doc(alias = "WP_PLUGIN_FEATURE_ENABLED")]
        const ENABLED = ffi::WP_PLUGIN_FEATURE_ENABLED as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for PluginFeatures {
    type GlibType = ffi::WpPluginFeatures;

    fn into_glib(self) -> ffi::WpPluginFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpPluginFeatures> for PluginFeatures {
    unsafe fn from_glib(value: ffi::WpPluginFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PluginFeatures {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_plugin_features_get_type()) }
    }
}

impl glib::value::ValueType for PluginFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PluginFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PluginFeatures {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpProxyFeatures")]
    pub struct ProxyFeatures: u32 {
        #[doc(alias = "WP_PROXY_FEATURE_BOUND")]
        const PROXY_FEATURE_BOUND = ffi::WP_PROXY_FEATURE_BOUND as u32;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_INFO")]
        const PIPEWIRE_OBJECT_FEATURE_INFO = ffi::WP_PIPEWIRE_OBJECT_FEATURE_INFO as u32;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROPS")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_PROPS = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROPS as u32;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_FORMAT")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_FORMAT = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_FORMAT as u32;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROFILE")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_PROFILE = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROFILE as u32;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PORT_CONFIG")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_PORT_CONFIG = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PORT_CONFIG as u32;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_ROUTE")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_ROUTE = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_ROUTE as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for ProxyFeatures {
    type GlibType = ffi::WpProxyFeatures;

    fn into_glib(self) -> ffi::WpProxyFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpProxyFeatures> for ProxyFeatures {
    unsafe fn from_glib(value: ffi::WpProxyFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ProxyFeatures {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_proxy_features_get_type()) }
    }
}

impl glib::value::ValueType for ProxyFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ProxyFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ProxyFeatures {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpSessionItemFeatures")]
    pub struct SessionItemFeatures: u32 {
        #[doc(alias = "WP_SESSION_ITEM_FEATURE_ACTIVE")]
        const ACTIVE = ffi::WP_SESSION_ITEM_FEATURE_ACTIVE as u32;
        #[doc(alias = "WP_SESSION_ITEM_FEATURE_EXPORTED")]
        const EXPORTED = ffi::WP_SESSION_ITEM_FEATURE_EXPORTED as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for SessionItemFeatures {
    type GlibType = ffi::WpSessionItemFeatures;

    fn into_glib(self) -> ffi::WpSessionItemFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpSessionItemFeatures> for SessionItemFeatures {
    unsafe fn from_glib(value: ffi::WpSessionItemFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for SessionItemFeatures {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_session_item_features_get_type()) }
    }
}

impl glib::value::ValueType for SessionItemFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SessionItemFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for SessionItemFeatures {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "WpSpaDeviceFeatures")]
    pub struct SpaDeviceFeatures: u32 {
        #[doc(alias = "WP_SPA_DEVICE_FEATURE_ENABLED")]
        const ENABLED = ffi::WP_SPA_DEVICE_FEATURE_ENABLED as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for SpaDeviceFeatures {
    type GlibType = ffi::WpSpaDeviceFeatures;

    fn into_glib(self) -> ffi::WpSpaDeviceFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpSpaDeviceFeatures> for SpaDeviceFeatures {
    unsafe fn from_glib(value: ffi::WpSpaDeviceFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for SpaDeviceFeatures {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_spa_device_features_get_type()) }
    }
}

impl glib::value::ValueType for SpaDeviceFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SpaDeviceFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for SpaDeviceFeatures {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

