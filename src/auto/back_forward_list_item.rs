// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitBackForwardListItem")]
    pub struct BackForwardListItem(Object<ffi::WebKitBackForwardListItem, ffi::WebKitBackForwardListItemClass>);

    match fn {
        type_ => || ffi::webkit_back_forward_list_item_get_type(),
    }
}

pub const NONE_BACK_FORWARD_LIST_ITEM: Option<&BackForwardListItem> = None;

pub trait BackForwardListItemExt: 'static {
    #[doc(alias = "webkit_back_forward_list_item_get_original_uri")]
    #[doc(alias = "get_original_uri")]
    fn original_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_back_forward_list_item_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_back_forward_list_item_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString>;
}

impl<O: IsA<BackForwardListItem>> BackForwardListItemExt for O {
    fn original_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_original_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for BackForwardListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BackForwardListItem")
    }
}
