// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib_sys;
use gtk_source_sys;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct SnippetContext(Object<gtk_source_sys::GtkSourceSnippetContext, gtk_source_sys::GtkSourceSnippetContextClass, SnippetContextClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_snippet_context_get_type(),
    }
}

impl SnippetContext {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    pub fn new() -> SnippetContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_snippet_context_new())
        }
    }
}

#[cfg(any(feature = "v5_0", feature = "dox"))]
impl Default for SnippetContext {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SNIPPET_CONTEXT: Option<&SnippetContext> = None;

pub trait SnippetContextExt: 'static {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn clear_variables(&self);

    fn expand(&self, input: &str) -> Option<GString>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_variable(&self, key: &str) -> Option<GString>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn set_constant(&self, key: &str, value: &str);

    fn set_line_prefix(&self, line_prefix: &str);

    fn set_tab_width(&self, tab_width: i32);

    fn set_use_spaces(&self, use_spaces: bool);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn set_variable(&self, key: &str, value: &str);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SnippetContext>> SnippetContextExt for O {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn clear_variables(&self) {
        unsafe {
            gtk_source_sys::gtk_source_snippet_context_clear_variables(self.as_ref().to_glib_none().0);
        }
    }

    fn expand(&self, input: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_snippet_context_expand(self.as_ref().to_glib_none().0, input.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_variable(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_snippet_context_get_variable(self.as_ref().to_glib_none().0, key.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn set_constant(&self, key: &str, value: &str) {
        unsafe {
            gtk_source_sys::gtk_source_snippet_context_set_constant(self.as_ref().to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_line_prefix(&self, line_prefix: &str) {
        unsafe {
            gtk_source_sys::gtk_source_snippet_context_set_line_prefix(self.as_ref().to_glib_none().0, line_prefix.to_glib_none().0);
        }
    }

    fn set_tab_width(&self, tab_width: i32) {
        unsafe {
            gtk_source_sys::gtk_source_snippet_context_set_tab_width(self.as_ref().to_glib_none().0, tab_width);
        }
    }

    fn set_use_spaces(&self, use_spaces: bool) {
        unsafe {
            gtk_source_sys::gtk_source_snippet_context_set_use_spaces(self.as_ref().to_glib_none().0, use_spaces.to_glib());
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn set_variable(&self, key: &str, value: &str) {
        unsafe {
            gtk_source_sys::gtk_source_snippet_context_set_variable(self.as_ref().to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourceSnippetContext, f: glib_sys::gpointer)
            where P: IsA<SnippetContext>
        {
            let f: &F = &*(f as *const F);
            f(&SnippetContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for SnippetContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SnippetContext")
    }
}
