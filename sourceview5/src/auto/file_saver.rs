// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buffer;
use crate::CompressionType;
use crate::Encoding;
use crate::File;
use crate::FileSaverFlags;
use crate::NewlineType;
use glib::object::Cast;
use glib::object::IsA;
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
    pub struct FileSaver(Object<ffi::GtkSourceFileSaver, ffi::GtkSourceFileSaverClass>);

    match fn {
        get_type => || ffi::gtk_source_file_saver_get_type(),
    }
}

impl FileSaver {
    pub fn new<P: IsA<Buffer>, Q: IsA<File>>(buffer: &P, file: &Q) -> FileSaver {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_saver_new(
                buffer.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn with_target<P: IsA<Buffer>, Q: IsA<File>, R: IsA<gio::File>>(
        buffer: &P,
        file: &Q,
        target_location: &R,
    ) -> FileSaver {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_saver_new_with_target(
                buffer.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                target_location.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn get_buffer(&self) -> Option<Buffer> {
        unsafe { from_glib_none(ffi::gtk_source_file_saver_get_buffer(self.to_glib_none().0)) }
    }

    pub fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_compression_type(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_encoding(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_file(&self) -> Option<File> {
        unsafe { from_glib_none(ffi::gtk_source_file_saver_get_file(self.to_glib_none().0)) }
    }

    pub fn get_flags(&self) -> FileSaverFlags {
        unsafe { from_glib(ffi::gtk_source_file_saver_get_flags(self.to_glib_none().0)) }
    }

    pub fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_location(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_newline_type(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn save_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static, R: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, progress_callback: Q, progress_callback_notify: Fn() + 'static, callback: R) {
    //    unsafe { TODO: call ffi:gtk_source_file_saver_save_async() }
    //}

    //
    //pub fn save_async_future<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: Q, progress_callback_notify: Fn() + 'static) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

    //let progress_callback = progress_callback.map(ToOwned::to_owned);
    //let progress_callback_notify = progress_callback_notify.map(ToOwned::to_owned);
    //Box_::pin(gio::GioFuture::new(self, move |obj, send| {
    //    let cancellable = gio::Cancellable::new();
    //    obj.save_async(
    //        io_priority,
    //        Some(&cancellable),
    //        progress_callback.as_ref().map(::std::borrow::Borrow::borrow),
    //        progress_callback_notify.as_ref().map(::std::borrow::Borrow::borrow),
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );

    //    cancellable
    //}))
    //}

    pub fn set_compression_type(&self, compression_type: CompressionType) {
        unsafe {
            ffi::gtk_source_file_saver_set_compression_type(
                self.to_glib_none().0,
                compression_type.to_glib(),
            );
        }
    }

    pub fn set_encoding(&self, encoding: Option<&Encoding>) {
        unsafe {
            ffi::gtk_source_file_saver_set_encoding(
                self.to_glib_none().0,
                encoding.to_glib_none().0,
            );
        }
    }

    pub fn set_flags(&self, flags: FileSaverFlags) {
        unsafe {
            ffi::gtk_source_file_saver_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    pub fn set_newline_type(&self, newline_type: NewlineType) {
        unsafe {
            ffi::gtk_source_file_saver_set_newline_type(
                self.to_glib_none().0,
                newline_type.to_glib(),
            );
        }
    }

    pub fn connect_property_compression_type_notify<F: Fn(&FileSaver) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_compression_type_trampoline<F: Fn(&FileSaver) + 'static>(
            this: *mut ffi::GtkSourceFileSaver,
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
                b"notify::compression-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_compression_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_encoding_notify<F: Fn(&FileSaver) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_encoding_trampoline<F: Fn(&FileSaver) + 'static>(
            this: *mut ffi::GtkSourceFileSaver,
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
                b"notify::encoding\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_encoding_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_flags_notify<F: Fn(&FileSaver) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&FileSaver) + 'static>(
            this: *mut ffi::GtkSourceFileSaver,
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
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_newline_type_notify<F: Fn(&FileSaver) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<F: Fn(&FileSaver) + 'static>(
            this: *mut ffi::GtkSourceFileSaver,
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
                b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_newline_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct FileSaverBuilder {
    buffer: Option<Buffer>,
    compression_type: Option<CompressionType>,
    encoding: Option<Encoding>,
    file: Option<File>,
    flags: Option<FileSaverFlags>,
    location: Option<gio::File>,
    newline_type: Option<NewlineType>,
}

impl FileSaverBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FileSaver {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref compression_type) = self.compression_type {
            properties.push(("compression-type", compression_type));
        }
        if let Some(ref encoding) = self.encoding {
            properties.push(("encoding", encoding));
        }
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
        }
        if let Some(ref location) = self.location {
            properties.push(("location", location));
        }
        if let Some(ref newline_type) = self.newline_type {
            properties.push(("newline-type", newline_type));
        }
        let ret = glib::Object::new(FileSaver::static_type(), &properties)
            .expect("object new")
            .downcast::<FileSaver>()
            .expect("downcast");
        ret
    }

    pub fn buffer<P: IsA<Buffer>>(mut self, buffer: &P) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn compression_type(mut self, compression_type: CompressionType) -> Self {
        self.compression_type = Some(compression_type);
        self
    }

    pub fn encoding(mut self, encoding: &Encoding) -> Self {
        self.encoding = Some(encoding.clone());
        self
    }

    pub fn file<P: IsA<File>>(mut self, file: &P) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    pub fn flags(mut self, flags: FileSaverFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn location<P: IsA<gio::File>>(mut self, location: &P) -> Self {
        self.location = Some(location.clone().upcast());
        self
    }

    pub fn newline_type(mut self, newline_type: NewlineType) -> Self {
        self.newline_type = Some(newline_type);
        self
    }
}

impl fmt::Display for FileSaver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileSaver")
    }
}
