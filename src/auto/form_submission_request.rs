// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitFormSubmissionRequest")]
    pub struct FormSubmissionRequest(Object<ffi::WebKitFormSubmissionRequest, ffi::WebKitFormSubmissionRequestClass>);

    match fn {
        type_ => || ffi::webkit_form_submission_request_get_type(),
    }
}

pub const NONE_FORM_SUBMISSION_REQUEST: Option<&FormSubmissionRequest> = None;

pub trait FormSubmissionRequestExt: 'static {
    //#[cfg_attr(feature = "v2_20", deprecated = "Since 2.20")]
    //#[doc(alias = "webkit_form_submission_request_get_text_fields")]
    //#[doc(alias = "get_text_fields")]
    //fn text_fields(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 };

    #[doc(alias = "webkit_form_submission_request_submit")]
    fn submit(&self);
}

impl<O: IsA<FormSubmissionRequest>> FormSubmissionRequestExt for O {
    //fn text_fields(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 } {
    //    unsafe { TODO: call ffi:webkit_form_submission_request_get_text_fields() }
    //}

    fn submit(&self) {
        unsafe {
            ffi::webkit_form_submission_request_submit(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for FormSubmissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FormSubmissionRequest")
    }
}
