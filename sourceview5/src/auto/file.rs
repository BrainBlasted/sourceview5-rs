// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CompressionType;
use crate::Encoding;
use crate::NewlineType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct File(Object<ffi::GtkSourceFile, ffi::GtkSourceFileClass>);

    match fn {
        get_type => || ffi::gtk_source_file_get_type(),
    }
}

impl File {
    pub fn new() -> File {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_source_file_new()) }
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct FileBuilder {
    location: Option<gio::File>,
}

impl FileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> File {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref location) = self.location {
            properties.push(("location", location));
        }
        let ret = glib::Object::new(File::static_type(), &properties)
            .expect("object new")
            .downcast::<File>()
            .expect("downcast");
        ret
    }

    pub fn location<P: IsA<gio::File>>(mut self, location: &P) -> Self {
        self.location = Some(location.clone().upcast());
        self
    }
}

pub const NONE_FILE: Option<&File> = None;

pub trait FileExt: 'static {
    fn check_file_on_disk(&self);

    fn get_compression_type(&self) -> CompressionType;

    fn get_encoding(&self) -> Option<Encoding>;

    fn get_location(&self) -> Option<gio::File>;

    fn get_newline_type(&self) -> NewlineType;

    fn is_deleted(&self) -> bool;

    fn is_externally_modified(&self) -> bool;

    fn is_local(&self) -> bool;

    fn is_readonly(&self) -> bool;

    fn set_location<P: IsA<gio::File>>(&self, location: Option<&P>);

    //fn set_mount_operation_factory(&self, callback: /*Unimplemented*/Fn(&File, /*Unimplemented*/Option<Fundamental: Pointer>) -> gio::MountOperation, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    fn get_property_read_only(&self) -> bool;

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<File>> FileExt for O {
    fn check_file_on_disk(&self) {
        unsafe {
            ffi::gtk_source_file_check_file_on_disk(self.as_ref().to_glib_none().0);
        }
    }

    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_get_compression_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_get_encoding(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_get_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_get_newline_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_file_is_deleted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_externally_modified(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_file_is_externally_modified(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_local(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_file_is_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_readonly(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_file_is_readonly(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_location<P: IsA<gio::File>>(&self, location: Option<&P>) {
        unsafe {
            ffi::gtk_source_file_set_location(
                self.as_ref().to_glib_none().0,
                location.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    //fn set_mount_operation_factory(&self, callback: /*Unimplemented*/Fn(&File, /*Unimplemented*/Option<Fundamental: Pointer>) -> gio::MountOperation, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gtk_source_file_set_mount_operation_factory() }
    //}

    fn get_property_read_only(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"read-only\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `read-only` getter")
                .unwrap()
        }
    }

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_compression_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::compression-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_compression_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_encoding_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::encoding\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_encoding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_location_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_location_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_newline_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_read_only_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::read-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_read_only_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("File")
    }
}
