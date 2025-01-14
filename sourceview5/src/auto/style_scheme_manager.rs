// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::StyleScheme;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct StyleSchemeManager(Object<ffi::GtkSourceStyleSchemeManager, ffi::GtkSourceStyleSchemeManagerClass>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_manager_get_type(),
    }
}

impl StyleSchemeManager {
    pub fn new() -> StyleSchemeManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_source_style_scheme_manager_new()) }
    }

    pub fn append_search_path(&self, path: &str) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_append_search_path(
                self.to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    pub fn force_rescan(&self) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_force_rescan(self.to_glib_none().0);
        }
    }

    pub fn get_scheme(&self, scheme_id: &str) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_manager_get_scheme(
                self.to_glib_none().0,
                scheme_id.to_glib_none().0,
            ))
        }
    }

    pub fn get_scheme_ids(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::gtk_source_style_scheme_manager_get_scheme_ids(self.to_glib_none().0),
            )
        }
    }

    pub fn get_search_path(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::gtk_source_style_scheme_manager_get_search_path(self.to_glib_none().0),
            )
        }
    }

    pub fn prepend_search_path(&self, path: &str) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_prepend_search_path(
                self.to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    pub fn set_search_path(&self, path: &[&str]) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_set_search_path(
                self.to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    pub fn get_default() -> Option<StyleSchemeManager> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_source_style_scheme_manager_get_default()) }
    }

    pub fn connect_property_scheme_ids_notify<F: Fn(&StyleSchemeManager) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scheme_ids_trampoline<F: Fn(&StyleSchemeManager) + 'static>(
            this: *mut ffi::GtkSourceStyleSchemeManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scheme-ids\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scheme_ids_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_search_path_notify<F: Fn(&StyleSchemeManager) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_path_trampoline<F: Fn(&StyleSchemeManager) + 'static>(
            this: *mut ffi::GtkSourceStyleSchemeManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_path_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for StyleSchemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct StyleSchemeManagerBuilder {
    search_path: Option<Vec<String>>,
}

impl StyleSchemeManagerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> StyleSchemeManager {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref search_path) = self.search_path {
            properties.push(("search-path", search_path));
        }
        let ret = glib::Object::new(StyleSchemeManager::static_type(), &properties)
            .expect("object new")
            .downcast::<StyleSchemeManager>()
            .expect("downcast");
        ret
    }

    pub fn search_path(mut self, search_path: Vec<String>) -> Self {
        self.search_path = Some(search_path);
        self
    }
}

impl fmt::Display for StyleSchemeManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleSchemeManager")
    }
}
