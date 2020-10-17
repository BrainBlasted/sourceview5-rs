// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use gtk_source_sys;
use std::fmt;
use Buffer;
use CompressionType;
use Encoding;
use File;
use NewlineType;

glib_wrapper! {
    pub struct FileLoader(Object<gtk_source_sys::GtkSourceFileLoader, gtk_source_sys::GtkSourceFileLoaderClass, FileLoaderClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_file_loader_get_type(),
    }
}

impl FileLoader {
    pub fn new<P: IsA<Buffer>, Q: IsA<File>>(buffer: &P, file: &Q) -> FileLoader {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_file_loader_new(buffer.as_ref().to_glib_none().0, file.as_ref().to_glib_none().0))
        }
    }

    pub fn from_stream<P: IsA<Buffer>, Q: IsA<File>, R: IsA<gio::InputStream>>(buffer: &P, file: &Q, stream: &R) -> FileLoader {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_file_loader_new_from_stream(buffer.as_ref().to_glib_none().0, file.as_ref().to_glib_none().0, stream.as_ref().to_glib_none().0))
        }
    }
}

#[derive(Clone, Default)]
pub struct FileLoaderBuilder {
    buffer: Option<Buffer>,
    file: Option<File>,
    input_stream: Option<gio::InputStream>,
    location: Option<gio::File>,
}

impl FileLoaderBuilder {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn build(self) -> FileLoader {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref input_stream) = self.input_stream {
            properties.push(("input-stream", input_stream));
        }
        if let Some(ref location) = self.location {
            properties.push(("location", location));
        }
        let ret = glib::Object::new(FileLoader::static_type(), &properties)
            .expect("object new")
            .downcast::<FileLoader>()
            .expect("downcast");
    ret
    }

    pub fn buffer<P: IsA<Buffer>>(mut self, buffer: &P) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn file<P: IsA<File>>(mut self, file: &P) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    pub fn input_stream<P: IsA<gio::InputStream>>(mut self, input_stream: &P) -> Self {
        self.input_stream = Some(input_stream.clone().upcast());
        self
    }

    pub fn location<P: IsA<gio::File>>(mut self, location: &P) -> Self {
        self.location = Some(location.clone().upcast());
        self
    }
}

pub const NONE_FILE_LOADER: Option<&FileLoader> = None;

pub trait FileLoaderExt: 'static {
    fn get_buffer(&self) -> Option<Buffer>;

    fn get_compression_type(&self) -> CompressionType;

    fn get_encoding(&self) -> Option<Encoding>;

    fn get_file(&self) -> Option<File>;

    fn get_input_stream(&self) -> Option<gio::InputStream>;

    fn get_location(&self) -> Option<gio::File>;

    fn get_newline_type(&self) -> NewlineType;

    //fn load_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static, R: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, progress_callback: Q, progress_callback_notify: Fn() + 'static, callback: R);

    //
    //fn load_async_future<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: Q, progress_callback_notify: Fn() + 'static) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn set_candidate_encodings(&self, candidate_encodings: &[&Encoding]);
}

impl<O: IsA<FileLoader>> FileLoaderExt for O {
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_loader_get_buffer(self.as_ref().to_glib_none().0))
        }
    }

    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_loader_get_compression_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_loader_get_encoding(self.as_ref().to_glib_none().0))
        }
    }

    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_loader_get_file(self.as_ref().to_glib_none().0))
        }
    }

    fn get_input_stream(&self) -> Option<gio::InputStream> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_loader_get_input_stream(self.as_ref().to_glib_none().0))
        }
    }

    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_loader_get_location(self.as_ref().to_glib_none().0))
        }
    }

    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_loader_get_newline_type(self.as_ref().to_glib_none().0))
        }
    }

    //fn load_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static, R: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, progress_callback: Q, progress_callback_notify: Fn() + 'static, callback: R) {
    //    unsafe { TODO: call gtk_source_sys:gtk_source_file_loader_load_async() }
    //}

    //
    //fn load_async_future<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: Q, progress_callback_notify: Fn() + 'static) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

        //let progress_callback = progress_callback.map(ToOwned::to_owned);
        //let progress_callback_notify = progress_callback_notify.map(ToOwned::to_owned);
        //Box_::pin(gio::GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    obj.load_async(
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

    fn set_candidate_encodings(&self, candidate_encodings: &[&Encoding]) {
        unsafe {
            gtk_source_sys::gtk_source_file_loader_set_candidate_encodings(self.as_ref().to_glib_none().0, candidate_encodings.to_glib_none().0);
        }
    }
}

impl fmt::Display for FileLoader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileLoader")
    }
}
