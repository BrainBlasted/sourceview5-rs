// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::StyleScheme;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct StyleSchemeChooser(Interface<ffi::GtkSourceStyleSchemeChooser>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_chooser_get_type(),
    }
}

pub const NONE_STYLE_SCHEME_CHOOSER: Option<&StyleSchemeChooser> = None;

pub trait StyleSchemeChooserExt: 'static {
    fn get_style_scheme(&self) -> Option<StyleScheme>;

    fn set_style_scheme(&self, scheme: &StyleScheme);

    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<StyleSchemeChooser>> StyleSchemeChooserExt for O {
    fn get_style_scheme(&self) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_chooser_get_style_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_style_scheme(&self, scheme: &StyleScheme) {
        unsafe {
            ffi::gtk_source_style_scheme_chooser_set_style_scheme(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_style_scheme_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceStyleSchemeChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StyleSchemeChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleSchemeChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::style-scheme\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_style_scheme_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleSchemeChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleSchemeChooser")
    }
}
