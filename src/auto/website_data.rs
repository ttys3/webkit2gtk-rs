// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::WebsiteDataTypes;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WebsiteData(Shared<ffi::WebKitWebsiteData>);

    match fn {
        ref => |ptr| ffi::webkit_website_data_ref(ptr),
        unref => |ptr| ffi::webkit_website_data_unref(ptr),
        type_ => || ffi::webkit_website_data_get_type(),
    }
}

impl WebsiteData {
    #[doc(alias = "webkit_website_data_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_website_data_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_website_data_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self, types: WebsiteDataTypes) -> u64 {
        unsafe { ffi::webkit_website_data_get_size(self.to_glib_none().0, types.into_glib()) }
    }

    #[doc(alias = "webkit_website_data_get_types")]
    #[doc(alias = "get_types")]
    pub fn types(&self) -> WebsiteDataTypes {
        unsafe { from_glib(ffi::webkit_website_data_get_types(self.to_glib_none().0)) }
    }
}
