// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use CompletionProvider;

glib_wrapper! {
    pub struct CompletionSnippets(Object<gtk_source_sys::GtkSourceCompletionSnippets, gtk_source_sys::GtkSourceCompletionSnippetsClass, CompletionSnippetsClass>) @implements CompletionProvider;

    match fn {
        get_type => || gtk_source_sys::gtk_source_completion_snippets_get_type(),
    }
}

impl CompletionSnippets {
    pub fn new() -> CompletionSnippets {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_source_sys::gtk_source_completion_snippets_new()) }
    }
}

impl Default for CompletionSnippets {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct CompletionSnippetsBuilder {
    priority: Option<i32>,
    title: Option<String>,
}

impl CompletionSnippetsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CompletionSnippets {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref priority) = self.priority {
            properties.push(("priority", priority));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        let ret = glib::Object::new(CompletionSnippets::static_type(), &properties)
            .expect("object new")
            .downcast::<CompletionSnippets>()
            .expect("downcast");
        ret
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

pub const NONE_COMPLETION_SNIPPETS: Option<&CompletionSnippets> = None;

pub trait CompletionSnippetsExt: 'static {
    fn get_property_priority(&self) -> i32;

    fn set_property_priority(&self, priority: i32);

    fn set_property_title(&self, title: Option<&str>);

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionSnippets>> CompletionSnippetsExt for O {
    fn get_property_priority(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"priority\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `priority` getter")
                .unwrap()
        }
    }

    fn set_property_priority(&self, priority: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"priority\0".as_ptr() as *const _,
                Value::from(&priority).to_glib_none().0,
            );
        }
    }

    fn set_property_title(&self, title: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"title\0".as_ptr() as *const _,
                Value::from(title).to_glib_none().0,
            );
        }
    }

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceCompletionSnippets,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CompletionSnippets>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionSnippets::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_priority_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceCompletionSnippets,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CompletionSnippets>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionSnippets::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CompletionSnippets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionSnippets")
    }
}
