// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use cairo;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use gio;
use gio_ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FaviconDatabase(Object<ffi::WebKitFaviconDatabase, ffi::WebKitFaviconDatabaseClass, FaviconDatabaseClass>);

    match fn {
        get_type => || ffi::webkit_favicon_database_get_type(),
    }
}

pub const NONE_FAVICON_DATABASE: Option<&FaviconDatabase> = None;

pub trait FaviconDatabaseExt: 'static {
    fn clear(&self);

    fn get_favicon<P: IsA<gio::Cancellable>, Q: FnOnce(Result<cairo::Surface, Error>) + Send + 'static>(&self, page_uri: &str, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn get_favicon_future(&self, page_uri: &str) -> Box_<futures_core::Future<Item = (Self, cairo::Surface), Error = (Self, Error)>> where Self: Sized + Clone;

    fn get_favicon_uri(&self, page_uri: &str) -> Option<GString>;

    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FaviconDatabase>> FaviconDatabaseExt for O {
    fn clear(&self) {
        unsafe {
            ffi::webkit_favicon_database_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn get_favicon<P: IsA<gio::Cancellable>, Q: FnOnce(Result<cairo::Surface, Error>) + Send + 'static>(&self, page_uri: &str, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn get_favicon_trampoline<Q: FnOnce(Result<cairo::Surface, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_favicon_database_get_favicon_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_favicon_trampoline::<Q>;
        unsafe {
            ffi::webkit_favicon_database_get_favicon(self.as_ref().to_glib_none().0, page_uri.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn get_favicon_future(&self, page_uri: &str) -> Box_<futures_core::Future<Item = (Self, cairo::Surface), Error = (Self, Error)>> where Self: Sized + Clone {
        use gio::GioFuture;
        use fragile::Fragile;

        let page_uri = String::from(page_uri);
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.get_favicon(
                &page_uri,
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn get_favicon_uri(&self, page_uri: &str) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_favicon_database_get_favicon_uri(self.as_ref().to_glib_none().0, page_uri.to_glib_none().0))
        }
    }

    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"favicon-changed\0".as_ptr() as *const _,
                Some(transmute(favicon_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn favicon_changed_trampoline<P, F: Fn(&P, &str, &str) + 'static>(this: *mut ffi::WebKitFaviconDatabase, page_uri: *mut libc::c_char, favicon_uri: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<FaviconDatabase> {
    let f: &F = &*(f as *const F);
    f(&FaviconDatabase::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(page_uri), &GString::from_glib_borrow(favicon_uri))
}

impl fmt::Display for FaviconDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FaviconDatabase")
    }
}
