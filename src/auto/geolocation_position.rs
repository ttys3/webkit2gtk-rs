// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_26", feature = "dox"))]
use glib::translate::*;
use webkit2_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GeolocationPosition(Boxed<webkit2_sys::WebKitGeolocationPosition>);

    match fn {
        copy => |ptr| webkit2_sys::webkit_geolocation_position_copy(mut_override(ptr)),
        free => |ptr| webkit2_sys::webkit_geolocation_position_free(ptr),
        get_type => || webkit2_sys::webkit_geolocation_position_get_type(),
    }
}

impl GeolocationPosition {
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    pub fn new(latitude: f64, longitude: f64, accuracy: f64) -> GeolocationPosition {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_geolocation_position_new(latitude, longitude, accuracy))
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    pub fn set_altitude(&mut self, altitude: f64) {
        unsafe {
            webkit2_sys::webkit_geolocation_position_set_altitude(self.to_glib_none_mut().0, altitude);
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    pub fn set_altitude_accuracy(&mut self, altitude_accuracy: f64) {
        unsafe {
            webkit2_sys::webkit_geolocation_position_set_altitude_accuracy(self.to_glib_none_mut().0, altitude_accuracy);
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    pub fn set_heading(&mut self, heading: f64) {
        unsafe {
            webkit2_sys::webkit_geolocation_position_set_heading(self.to_glib_none_mut().0, heading);
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    pub fn set_speed(&mut self, speed: f64) {
        unsafe {
            webkit2_sys::webkit_geolocation_position_set_speed(self.to_glib_none_mut().0, speed);
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    pub fn set_timestamp(&mut self, timestamp: u64) {
        unsafe {
            webkit2_sys::webkit_geolocation_position_set_timestamp(self.to_glib_none_mut().0, timestamp);
        }
    }
}
