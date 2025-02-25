// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Encoding(Boxed<ffi::GtkSourceEncoding>);

    match fn {
        copy => |ptr| ffi::gtk_source_encoding_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_source_encoding_free(ptr),
        get_type => || ffi::gtk_source_encoding_get_type(),
    }
}

impl Encoding {
    pub fn get_charset(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_source_encoding_get_charset(self.to_glib_none().0)) }
    }

    pub fn get_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_source_encoding_get_name(self.to_glib_none().0)) }
    }

    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gtk_source_encoding_to_string(self.to_glib_none().0)) }
    }

    pub fn get_all() -> Vec<Encoding> {
        assert_initialized_main_thread!();
        unsafe { FromGlibPtrContainer::from_glib_container(ffi::gtk_source_encoding_get_all()) }
    }

    pub fn get_current() -> Option<Encoding> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_source_encoding_get_current()) }
    }

    pub fn get_default_candidates() -> Vec<Encoding> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::gtk_source_encoding_get_default_candidates(),
            )
        }
    }

    pub fn get_from_charset(charset: &str) -> Option<Encoding> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_source_encoding_get_from_charset(
                charset.to_glib_none().0,
            ))
        }
    }

    pub fn get_utf8() -> Option<Encoding> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_source_encoding_get_utf8()) }
    }
}

impl fmt::Display for Encoding {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
