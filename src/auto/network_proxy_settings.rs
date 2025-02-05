// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NetworkProxySettings(Boxed<ffi::WebKitNetworkProxySettings>);

    match fn {
        copy => |ptr| ffi::webkit_network_proxy_settings_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_network_proxy_settings_free(ptr),
        type_ => || ffi::webkit_network_proxy_settings_get_type(),
    }
}

impl NetworkProxySettings {
    #[doc(alias = "webkit_network_proxy_settings_add_proxy_for_scheme")]
    pub fn add_proxy_for_scheme(&mut self, scheme: &str, proxy_uri: &str) {
        unsafe {
            ffi::webkit_network_proxy_settings_add_proxy_for_scheme(
                self.to_glib_none_mut().0,
                scheme.to_glib_none().0,
                proxy_uri.to_glib_none().0,
            );
        }
    }
}
