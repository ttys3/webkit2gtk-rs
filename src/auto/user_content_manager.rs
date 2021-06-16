// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use crate::JavascriptResult;
use crate::UserScript;
use crate::UserStyleSheet;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitUserContentManager")]
    pub struct UserContentManager(Object<ffi::WebKitUserContentManager, ffi::WebKitUserContentManagerClass>);

    match fn {
        type_ => || ffi::webkit_user_content_manager_get_type(),
    }
}

impl UserContentManager {
    #[doc(alias = "webkit_user_content_manager_new")]
    pub fn new() -> UserContentManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_user_content_manager_new()) }
    }
}

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
impl Default for UserContentManager {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_USER_CONTENT_MANAGER: Option<&UserContentManager> = None;

pub trait UserContentManagerExt: 'static {
    //#[cfg(any(feature = "v2_24", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    //#[doc(alias = "webkit_user_content_manager_add_filter")]
    //fn add_filter(&self, filter: /*Ignored*/&UserContentFilter);

    #[doc(alias = "webkit_user_content_manager_add_script")]
    fn add_script(&self, script: &UserScript);

    #[doc(alias = "webkit_user_content_manager_add_style_sheet")]
    fn add_style_sheet(&self, stylesheet: &UserStyleSheet);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_user_content_manager_register_script_message_handler")]
    fn register_script_message_handler(&self, name: &str) -> bool;

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_user_content_manager_register_script_message_handler_in_world")]
    fn register_script_message_handler_in_world(&self, name: &str, world_name: &str) -> bool;

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    #[doc(alias = "webkit_user_content_manager_remove_all_filters")]
    fn remove_all_filters(&self);

    #[doc(alias = "webkit_user_content_manager_remove_all_scripts")]
    fn remove_all_scripts(&self);

    #[doc(alias = "webkit_user_content_manager_remove_all_style_sheets")]
    fn remove_all_style_sheets(&self);

    //#[doc(alias = "webkit_user_content_manager_remove_filter")]
    //fn remove_filter(&self, filter: /*Ignored*/&UserContentFilter);

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "webkit_user_content_manager_remove_filter_by_id")]
    fn remove_filter_by_id(&self, filter_id: &str);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_user_content_manager_unregister_script_message_handler")]
    fn unregister_script_message_handler(&self, name: &str);

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_user_content_manager_unregister_script_message_handler_in_world")]
    fn unregister_script_message_handler_in_world(&self, name: &str, world_name: &str);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "script-message-received")]
    fn connect_script_message_received<F: Fn(&Self, &JavascriptResult) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<UserContentManager>> UserContentManagerExt for O {
    //#[cfg(any(feature = "v2_24", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    //fn add_filter(&self, filter: /*Ignored*/&UserContentFilter) {
    //    unsafe { TODO: call ffi:webkit_user_content_manager_add_filter() }
    //}

    fn add_script(&self, script: &UserScript) {
        unsafe {
            ffi::webkit_user_content_manager_add_script(
                self.as_ref().to_glib_none().0,
                script.to_glib_none().0,
            );
        }
    }

    fn add_style_sheet(&self, stylesheet: &UserStyleSheet) {
        unsafe {
            ffi::webkit_user_content_manager_add_style_sheet(
                self.as_ref().to_glib_none().0,
                stylesheet.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn register_script_message_handler(&self, name: &str) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_user_content_manager_register_script_message_handler(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    fn register_script_message_handler_in_world(&self, name: &str, world_name: &str) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_user_content_manager_register_script_message_handler_in_world(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                    world_name.to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    fn remove_all_filters(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_filters(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_all_scripts(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_scripts(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_all_style_sheets(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_style_sheets(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    //fn remove_filter(&self, filter: /*Ignored*/&UserContentFilter) {
    //    unsafe { TODO: call ffi:webkit_user_content_manager_remove_filter() }
    //}

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn remove_filter_by_id(&self, filter_id: &str) {
        unsafe {
            ffi::webkit_user_content_manager_remove_filter_by_id(
                self.as_ref().to_glib_none().0,
                filter_id.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn unregister_script_message_handler(&self, name: &str) {
        unsafe {
            ffi::webkit_user_content_manager_unregister_script_message_handler(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    fn unregister_script_message_handler_in_world(&self, name: &str, world_name: &str) {
        unsafe {
            ffi::webkit_user_content_manager_unregister_script_message_handler_in_world(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                world_name.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn connect_script_message_received<F: Fn(&Self, &JavascriptResult) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn script_message_received_trampoline<
            P: IsA<UserContentManager>,
            F: Fn(&P, &JavascriptResult) + 'static,
        >(
            this: *mut ffi::WebKitUserContentManager,
            js_result: *mut ffi::WebKitJavascriptResult,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &UserContentManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(js_result),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name =
                detail.map(|name| format!("script-message-received::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"script-message-received\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    script_message_received_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for UserContentManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UserContentManager")
    }
}
