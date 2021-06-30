// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use crate::AutomationSession;
use crate::CacheModel;
use crate::CookieManager;
use crate::Download;
use crate::FaviconDatabase;
#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
use crate::GeolocationManager;
use crate::Plugin;
#[cfg(any(feature = "v2_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
use crate::ProcessModel;
use crate::SecurityManager;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::SecurityOrigin;
use crate::TLSErrorsPolicy;
use crate::URISchemeRequest;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
use crate::UserMessage;
#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
use crate::WebsiteDataManager;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitWebContext")]
    pub struct WebContext(Object<ffi::WebKitWebContext, ffi::WebKitWebContextClass>);

    match fn {
        type_ => || ffi::webkit_web_context_get_type(),
    }
}

impl WebContext {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_web_context_new")]
    pub fn new() -> WebContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new())
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_web_context_new_ephemeral")]
    pub fn new_ephemeral() -> WebContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new_ephemeral())
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    #[doc(alias = "webkit_web_context_new_with_website_data_manager")]
    #[doc(alias = "new_with_website_data_manager")]
    pub fn with_website_data_manager<P: IsA<WebsiteDataManager>>(manager: &P) -> WebContext {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new_with_website_data_manager(manager.as_ref().to_glib_none().0))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`WebContext`] objects.
            ///
            /// This method returns an instance of [`WebContextBuilder`] which can be used to create [`WebContext`] objects.
            pub fn builder() -> WebContextBuilder {
                WebContextBuilder::default()
            }
        

    #[doc(alias = "webkit_web_context_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> Option<WebContext> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_default())
        }
    }
}

#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
impl Default for WebContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`WebContext`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct WebContextBuilder {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[cfg_attr(feature = "v2_10", deprecated = "Since 2.10")]
    local_storage_directory: Option<String>,
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    process_swap_on_cross_site_navigation_enabled: Option<bool>,
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    use_system_appearance_for_scrollbars: Option<bool>,
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    website_data_manager: Option<WebsiteDataManager>,
}

impl WebContextBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`WebContextBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`WebContext`].
    pub fn build(self) -> WebContext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_8", feature = "dox"))]
if let Some(ref local_storage_directory) = self.local_storage_directory {
                properties.push(("local-storage-directory", local_storage_directory));
            }
        #[cfg(any(feature = "v2_28", feature = "dox"))]
if let Some(ref process_swap_on_cross_site_navigation_enabled) = self.process_swap_on_cross_site_navigation_enabled {
                properties.push(("process-swap-on-cross-site-navigation-enabled", process_swap_on_cross_site_navigation_enabled));
            }
        #[cfg(any(feature = "v2_30", feature = "dox"))]
if let Some(ref use_system_appearance_for_scrollbars) = self.use_system_appearance_for_scrollbars {
                properties.push(("use-system-appearance-for-scrollbars", use_system_appearance_for_scrollbars));
            }
        #[cfg(any(feature = "v2_10", feature = "dox"))]
if let Some(ref website_data_manager) = self.website_data_manager {
                properties.push(("website-data-manager", website_data_manager));
            }
        glib::Object::new::<WebContext>(&properties)
                .expect("Failed to create an instance of WebContext")

    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[cfg_attr(feature = "v2_10", deprecated = "Since 2.10")]
    pub fn local_storage_directory(mut self, local_storage_directory: &str) -> Self {
        self.local_storage_directory = Some(local_storage_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    pub fn process_swap_on_cross_site_navigation_enabled(mut self, process_swap_on_cross_site_navigation_enabled: bool) -> Self {
        self.process_swap_on_cross_site_navigation_enabled = Some(process_swap_on_cross_site_navigation_enabled);
        self
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    pub fn use_system_appearance_for_scrollbars(mut self, use_system_appearance_for_scrollbars: bool) -> Self {
        self.use_system_appearance_for_scrollbars = Some(use_system_appearance_for_scrollbars);
        self
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub fn website_data_manager<P: IsA<WebsiteDataManager>>(mut self, website_data_manager: &P) -> Self {
        self.website_data_manager = Some(website_data_manager.clone().upcast());
        self
    }
}

pub const NONE_WEB_CONTEXT: Option<&WebContext> = None;

pub trait WebContextExt: 'static {
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "webkit_web_context_add_path_to_sandbox")]
    fn add_path_to_sandbox<P: AsRef<std::path::Path>>(&self, path: P, read_only: bool);

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    #[doc(alias = "webkit_web_context_allow_tls_certificate_for_host")]
    fn allow_tls_certificate_for_host<P: IsA<gio::TlsCertificate>>(&self, certificate: &P, host: &str);

    #[doc(alias = "webkit_web_context_clear_cache")]
    fn clear_cache(&self);

    #[doc(alias = "webkit_web_context_download_uri")]
    fn download_uri(&self, uri: &str) -> Option<Download>;

    #[doc(alias = "webkit_web_context_get_cache_model")]
    #[doc(alias = "get_cache_model")]
    fn cache_model(&self) -> CacheModel;

    #[doc(alias = "webkit_web_context_get_cookie_manager")]
    #[doc(alias = "get_cookie_manager")]
    fn cookie_manager(&self) -> Option<CookieManager>;

    #[doc(alias = "webkit_web_context_get_favicon_database")]
    #[doc(alias = "get_favicon_database")]
    fn favicon_database(&self) -> Option<FaviconDatabase>;

    #[doc(alias = "webkit_web_context_get_favicon_database_directory")]
    #[doc(alias = "get_favicon_database_directory")]
    fn favicon_database_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "webkit_web_context_get_geolocation_manager")]
    #[doc(alias = "get_geolocation_manager")]
    fn geolocation_manager(&self) -> Option<GeolocationManager>;

    #[doc(alias = "webkit_web_context_get_plugins")]
    #[doc(alias = "get_plugins")]
    fn plugins<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<Plugin>, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    
    fn plugins_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<Plugin>, glib::Error>> + 'static>>;

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    #[doc(alias = "webkit_web_context_get_process_model")]
    #[doc(alias = "get_process_model")]
    fn process_model(&self) -> ProcessModel;

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "webkit_web_context_get_sandbox_enabled")]
    #[doc(alias = "get_sandbox_enabled")]
    fn is_sandbox_enabled(&self) -> bool;

    #[doc(alias = "webkit_web_context_get_security_manager")]
    #[doc(alias = "get_security_manager")]
    fn security_manager(&self) -> Option<SecurityManager>;

    #[doc(alias = "webkit_web_context_get_spell_checking_enabled")]
    #[doc(alias = "get_spell_checking_enabled")]
    fn is_spell_checking_enabled(&self) -> bool;

    #[doc(alias = "webkit_web_context_get_spell_checking_languages")]
    #[doc(alias = "get_spell_checking_languages")]
    fn spell_checking_languages(&self) -> Vec<glib::GString>;

    #[doc(alias = "webkit_web_context_get_tls_errors_policy")]
    #[doc(alias = "get_tls_errors_policy")]
    fn tls_errors_policy(&self) -> TLSErrorsPolicy;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_web_context_get_use_system_appearance_for_scrollbars")]
    #[doc(alias = "get_use_system_appearance_for_scrollbars")]
    fn uses_system_appearance_for_scrollbars(&self) -> bool;

    #[cfg_attr(feature = "v2_26", deprecated = "Since 2.26")]
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    #[doc(alias = "webkit_web_context_get_web_process_count_limit")]
    #[doc(alias = "get_web_process_count_limit")]
    fn web_process_count_limit(&self) -> u32;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    #[doc(alias = "webkit_web_context_get_website_data_manager")]
    #[doc(alias = "get_website_data_manager")]
    fn website_data_manager(&self) -> Option<WebsiteDataManager>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_web_context_initialize_notification_permissions")]
    fn initialize_notification_permissions(&self, allowed_origins: &[&SecurityOrigin], disallowed_origins: &[&SecurityOrigin]);

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_web_context_is_automation_allowed")]
    fn is_automation_allowed(&self) -> bool;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_web_context_is_ephemeral")]
    fn is_ephemeral(&self) -> bool;

    #[doc(alias = "webkit_web_context_prefetch_dns")]
    fn prefetch_dns(&self, hostname: &str);

    #[doc(alias = "webkit_web_context_register_uri_scheme")]
    fn register_uri_scheme<P: Fn(&URISchemeRequest) + 'static>(&self, scheme: &str, callback: P);

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "webkit_web_context_send_message_to_all_extensions")]
    fn send_message_to_all_extensions<P: IsA<UserMessage>>(&self, message: &P);

    #[doc(alias = "webkit_web_context_set_additional_plugins_directory")]
    fn set_additional_plugins_directory(&self, directory: &str);

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_web_context_set_automation_allowed")]
    fn set_automation_allowed(&self, allowed: bool);

    #[doc(alias = "webkit_web_context_set_cache_model")]
    fn set_cache_model(&self, cache_model: CacheModel);

    #[cfg_attr(feature = "v2_10", deprecated = "Since 2.10")]
    #[doc(alias = "webkit_web_context_set_disk_cache_directory")]
    fn set_disk_cache_directory(&self, directory: &str);

    #[doc(alias = "webkit_web_context_set_favicon_database_directory")]
    fn set_favicon_database_directory(&self, path: Option<&str>);

    #[doc(alias = "webkit_web_context_set_preferred_languages")]
    fn set_preferred_languages(&self, languages: &[&str]);

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    #[doc(alias = "webkit_web_context_set_process_model")]
    fn set_process_model(&self, process_model: ProcessModel);

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "webkit_web_context_set_sandbox_enabled")]
    fn set_sandbox_enabled(&self, enabled: bool);

    #[doc(alias = "webkit_web_context_set_spell_checking_enabled")]
    fn set_spell_checking_enabled(&self, enabled: bool);

    #[doc(alias = "webkit_web_context_set_spell_checking_languages")]
    fn set_spell_checking_languages(&self, languages: &[&str]);

    #[doc(alias = "webkit_web_context_set_tls_errors_policy")]
    fn set_tls_errors_policy(&self, policy: TLSErrorsPolicy);

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_web_context_set_use_system_appearance_for_scrollbars")]
    fn set_use_system_appearance_for_scrollbars(&self, enabled: bool);

    #[doc(alias = "webkit_web_context_set_web_extensions_directory")]
    fn set_web_extensions_directory(&self, directory: &str);

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    #[doc(alias = "webkit_web_context_set_web_extensions_initialization_user_data")]
    fn set_web_extensions_initialization_user_data(&self, user_data: &glib::Variant);

    #[cfg_attr(feature = "v2_26", deprecated = "Since 2.26")]
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    #[doc(alias = "webkit_web_context_set_web_process_count_limit")]
    fn set_web_process_count_limit(&self, limit: u32);

    #[cfg_attr(feature = "v2_10", deprecated = "Since 2.10")]
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "local-storage-directory")]
    fn local_storage_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "process-swap-on-cross-site-navigation-enabled")]
    fn is_process_swap_on_cross_site_navigation_enabled(&self) -> bool;

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "automation-started")]
    fn connect_automation_started<F: Fn(&Self, &AutomationSession) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "download-started")]
    fn connect_download_started<F: Fn(&Self, &Download) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "initialize-notification-permissions")]
    fn connect_initialize_notification_permissions<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    #[doc(alias = "initialize-web-extensions")]
    fn connect_initialize_web_extensions<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "user-message-received")]
    fn connect_user_message_received<F: Fn(&Self, &UserMessage) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "use-system-appearance-for-scrollbars")]
    fn connect_use_system_appearance_for_scrollbars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebContext>> WebContextExt for O {
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn add_path_to_sandbox<P: AsRef<std::path::Path>>(&self, path: P, read_only: bool) {
        unsafe {
            ffi::webkit_web_context_add_path_to_sandbox(self.as_ref().to_glib_none().0, path.as_ref().to_glib_none().0, read_only.into_glib());
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    fn allow_tls_certificate_for_host<P: IsA<gio::TlsCertificate>>(&self, certificate: &P, host: &str) {
        unsafe {
            ffi::webkit_web_context_allow_tls_certificate_for_host(self.as_ref().to_glib_none().0, certificate.as_ref().to_glib_none().0, host.to_glib_none().0);
        }
    }

    fn clear_cache(&self) {
        unsafe {
            ffi::webkit_web_context_clear_cache(self.as_ref().to_glib_none().0);
        }
    }

    fn download_uri(&self, uri: &str) -> Option<Download> {
        unsafe {
            from_glib_full(ffi::webkit_web_context_download_uri(self.as_ref().to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn cache_model(&self) -> CacheModel {
        unsafe {
            from_glib(ffi::webkit_web_context_get_cache_model(self.as_ref().to_glib_none().0))
        }
    }

    fn cookie_manager(&self) -> Option<CookieManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_cookie_manager(self.as_ref().to_glib_none().0))
        }
    }

    fn favicon_database(&self) -> Option<FaviconDatabase> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_favicon_database(self.as_ref().to_glib_none().0))
        }
    }

    fn favicon_database_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_favicon_database_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn geolocation_manager(&self) -> Option<GeolocationManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_geolocation_manager(self.as_ref().to_glib_none().0))
        }
    }

    fn plugins<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<Plugin>, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn plugins_trampoline<Q: FnOnce(Result<Vec<Plugin>, glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_web_context_get_plugins_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = plugins_trampoline::<Q>;
        unsafe {
            ffi::webkit_web_context_get_plugins(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn plugins_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<Plugin>, glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.plugins(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    fn process_model(&self) -> ProcessModel {
        unsafe {
            from_glib(ffi::webkit_web_context_get_process_model(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn is_sandbox_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_get_sandbox_enabled(self.as_ref().to_glib_none().0))
        }
    }

    fn security_manager(&self) -> Option<SecurityManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_security_manager(self.as_ref().to_glib_none().0))
        }
    }

    fn is_spell_checking_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_get_spell_checking_enabled(self.as_ref().to_glib_none().0))
        }
    }

    fn spell_checking_languages(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_web_context_get_spell_checking_languages(self.as_ref().to_glib_none().0))
        }
    }

    fn tls_errors_policy(&self) -> TLSErrorsPolicy {
        unsafe {
            from_glib(ffi::webkit_web_context_get_tls_errors_policy(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn uses_system_appearance_for_scrollbars(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_get_use_system_appearance_for_scrollbars(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    fn web_process_count_limit(&self) -> u32 {
        unsafe {
            ffi::webkit_web_context_get_web_process_count_limit(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    fn website_data_manager(&self) -> Option<WebsiteDataManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_website_data_manager(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn initialize_notification_permissions(&self, allowed_origins: &[&SecurityOrigin], disallowed_origins: &[&SecurityOrigin]) {
        unsafe {
            ffi::webkit_web_context_initialize_notification_permissions(self.as_ref().to_glib_none().0, allowed_origins.to_glib_none().0, disallowed_origins.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn is_automation_allowed(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_is_automation_allowed(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn is_ephemeral(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_is_ephemeral(self.as_ref().to_glib_none().0))
        }
    }

    fn prefetch_dns(&self, hostname: &str) {
        unsafe {
            ffi::webkit_web_context_prefetch_dns(self.as_ref().to_glib_none().0, hostname.to_glib_none().0);
        }
    }

    fn register_uri_scheme<P: Fn(&URISchemeRequest) + 'static>(&self, scheme: &str, callback: P) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&URISchemeRequest) + 'static>(request: *mut ffi::WebKitURISchemeRequest, user_data: glib::ffi::gpointer) {
            let request = from_glib_borrow(request);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&request);
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn user_data_destroy_func_func<P: Fn(&URISchemeRequest) + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(user_data_destroy_func_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::webkit_web_context_register_uri_scheme(self.as_ref().to_glib_none().0, scheme.to_glib_none().0, callback, Box_::into_raw(super_callback0) as *mut _, destroy_call4);
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn send_message_to_all_extensions<P: IsA<UserMessage>>(&self, message: &P) {
        unsafe {
            ffi::webkit_web_context_send_message_to_all_extensions(self.as_ref().to_glib_none().0, message.as_ref().to_glib_none().0);
        }
    }

    fn set_additional_plugins_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_additional_plugins_directory(self.as_ref().to_glib_none().0, directory.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn set_automation_allowed(&self, allowed: bool) {
        unsafe {
            ffi::webkit_web_context_set_automation_allowed(self.as_ref().to_glib_none().0, allowed.into_glib());
        }
    }

    fn set_cache_model(&self, cache_model: CacheModel) {
        unsafe {
            ffi::webkit_web_context_set_cache_model(self.as_ref().to_glib_none().0, cache_model.into_glib());
        }
    }

    fn set_disk_cache_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_disk_cache_directory(self.as_ref().to_glib_none().0, directory.to_glib_none().0);
        }
    }

    fn set_favicon_database_directory(&self, path: Option<&str>) {
        unsafe {
            ffi::webkit_web_context_set_favicon_database_directory(self.as_ref().to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn set_preferred_languages(&self, languages: &[&str]) {
        unsafe {
            ffi::webkit_web_context_set_preferred_languages(self.as_ref().to_glib_none().0, languages.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    fn set_process_model(&self, process_model: ProcessModel) {
        unsafe {
            ffi::webkit_web_context_set_process_model(self.as_ref().to_glib_none().0, process_model.into_glib());
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn set_sandbox_enabled(&self, enabled: bool) {
        unsafe {
            ffi::webkit_web_context_set_sandbox_enabled(self.as_ref().to_glib_none().0, enabled.into_glib());
        }
    }

    fn set_spell_checking_enabled(&self, enabled: bool) {
        unsafe {
            ffi::webkit_web_context_set_spell_checking_enabled(self.as_ref().to_glib_none().0, enabled.into_glib());
        }
    }

    fn set_spell_checking_languages(&self, languages: &[&str]) {
        unsafe {
            ffi::webkit_web_context_set_spell_checking_languages(self.as_ref().to_glib_none().0, languages.to_glib_none().0);
        }
    }

    fn set_tls_errors_policy(&self, policy: TLSErrorsPolicy) {
        unsafe {
            ffi::webkit_web_context_set_tls_errors_policy(self.as_ref().to_glib_none().0, policy.into_glib());
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn set_use_system_appearance_for_scrollbars(&self, enabled: bool) {
        unsafe {
            ffi::webkit_web_context_set_use_system_appearance_for_scrollbars(self.as_ref().to_glib_none().0, enabled.into_glib());
        }
    }

    fn set_web_extensions_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_web_extensions_directory(self.as_ref().to_glib_none().0, directory.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    fn set_web_extensions_initialization_user_data(&self, user_data: &glib::Variant) {
        unsafe {
            ffi::webkit_web_context_set_web_extensions_initialization_user_data(self.as_ref().to_glib_none().0, user_data.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    fn set_web_process_count_limit(&self, limit: u32) {
        unsafe {
            ffi::webkit_web_context_set_web_process_count_limit(self.as_ref().to_glib_none().0, limit);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn local_storage_directory(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"local-storage-directory\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `local-storage-directory` getter")
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn is_process_swap_on_cross_site_navigation_enabled(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"process-swap-on-cross-site-navigation-enabled\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `process-swap-on-cross-site-navigation-enabled` getter")
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn connect_automation_started<F: Fn(&Self, &AutomationSession) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn automation_started_trampoline<P: IsA<WebContext>, F: Fn(&P, &AutomationSession) + 'static>(this: *mut ffi::WebKitWebContext, session: *mut ffi::WebKitAutomationSession, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebContext::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(session))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"automation-started\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(automation_started_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_download_started<F: Fn(&Self, &Download) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn download_started_trampoline<P: IsA<WebContext>, F: Fn(&P, &Download) + 'static>(this: *mut ffi::WebKitWebContext, download: *mut ffi::WebKitDownload, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebContext::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(download))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"download-started\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(download_started_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn connect_initialize_notification_permissions<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn initialize_notification_permissions_trampoline<P: IsA<WebContext>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitWebContext, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"initialize-notification-permissions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(initialize_notification_permissions_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    fn connect_initialize_web_extensions<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn initialize_web_extensions_trampoline<P: IsA<WebContext>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitWebContext, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"initialize-web-extensions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(initialize_web_extensions_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_user_message_received<F: Fn(&Self, &UserMessage) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn user_message_received_trampoline<P: IsA<WebContext>, F: Fn(&P, &UserMessage) -> bool + 'static>(this: *mut ffi::WebKitWebContext, message: *mut ffi::WebKitUserMessage, f: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(WebContext::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(message)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"user-message-received\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(user_message_received_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn connect_use_system_appearance_for_scrollbars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_system_appearance_for_scrollbars_trampoline<P: IsA<WebContext>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitWebContext, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-system-appearance-for-scrollbars\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_use_system_appearance_for_scrollbars_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for WebContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebContext")
    }
}
