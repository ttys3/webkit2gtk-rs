// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use gtk;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use gtk_ffi;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct PrintCustomWidget(Object<ffi::WebKitPrintCustomWidget, ffi::WebKitPrintCustomWidgetClass, PrintCustomWidgetClass>);

    match fn {
        get_type => || ffi::webkit_print_custom_widget_get_type(),
    }
}

impl PrintCustomWidget {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn new<P: IsA<gtk::Widget>>(widget: &P, title: &str) -> PrintCustomWidget {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_print_custom_widget_new(widget.as_ref().to_glib_none().0, title.to_glib_none().0))
        }
    }
}

pub const NONE_PRINT_CUSTOM_WIDGET: Option<&PrintCustomWidget> = None;

pub trait PrintCustomWidgetExt: 'static {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_title(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_widget(&self) -> Option<gtk::Widget>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_update<F: Fn(&Self, &gtk::PageSetup, &gtk::PrintSettings) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintCustomWidget>> PrintCustomWidgetExt for O {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::webkit_print_custom_widget_get_title(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::webkit_print_custom_widget_get_widget(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"apply\0".as_ptr() as *const _,
                Some(transmute(apply_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_update<F: Fn(&Self, &gtk::PageSetup, &gtk::PrintSettings) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"update\0".as_ptr() as *const _,
                Some(transmute(update_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn apply_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitPrintCustomWidget, f: glib_ffi::gpointer)
where P: IsA<PrintCustomWidget> {
    let f: &F = &*(f as *const F);
    f(&PrintCustomWidget::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn update_trampoline<P, F: Fn(&P, &gtk::PageSetup, &gtk::PrintSettings) + 'static>(this: *mut ffi::WebKitPrintCustomWidget, page_setup: *mut gtk_ffi::GtkPageSetup, print_settings: *mut gtk_ffi::GtkPrintSettings, f: glib_ffi::gpointer)
where P: IsA<PrintCustomWidget> {
    let f: &F = &*(f as *const F);
    f(&PrintCustomWidget::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(page_setup), &from_glib_borrow(print_settings))
}

impl fmt::Display for PrintCustomWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PrintCustomWidget")
    }
}
