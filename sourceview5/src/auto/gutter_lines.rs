// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use gtk;
use gtk_source_sys;
use std::fmt;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use std::mem;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use GutterRendererAlignmentMode;

glib_wrapper! {
    pub struct GutterLines(Object<gtk_source_sys::GtkSourceGutterLines, gtk_source_sys::GtkSourceGutterLinesClass, GutterLinesClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_gutter_lines_get_type(),
    }
}

pub const NONE_GUTTER_LINES: Option<&GutterLines> = None;

pub trait GutterLinesExt: 'static {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn add_class(&self, line: u32, name: &str);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn add_qclass(&self, line: u32, qname: glib::Quark);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_buffer(&self) -> Option<gtk::TextBuffer>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_first(&self) -> u32;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_iter_at_line(&self, line: u32) -> gtk::TextIter;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_last(&self) -> u32;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_line_yrange(&self, line: u32, mode: GutterRendererAlignmentMode) -> (i32, i32);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_view(&self) -> Option<gtk::TextView>;

    fn get_yrange(&self, line: u32, line_y: u32, line_height: u32);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn has_class(&self, line: u32, name: &str) -> bool;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn has_qclass(&self, line: u32, qname: glib::Quark) -> bool;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn is_cursor(&self, line: u32) -> bool;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn is_prelit(&self, line: u32) -> bool;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn is_selected(&self, line: u32) -> bool;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn remove_class(&self, line: u32, name: &str);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn remove_qclass(&self, line: u32, qname: glib::Quark);
}

impl<O: IsA<GutterLines>> GutterLinesExt for O {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn add_class(&self, line: u32, name: &str) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_lines_add_class(self.as_ref().to_glib_none().0, line, name.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn add_qclass(&self, line: u32, qname: glib::Quark) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_lines_add_qclass(self.as_ref().to_glib_none().0, line, qname.to_glib());
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_buffer(&self) -> Option<gtk::TextBuffer> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_gutter_lines_get_buffer(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_first(&self) -> u32 {
        unsafe {
            gtk_source_sys::gtk_source_gutter_lines_get_first(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_iter_at_line(&self, line: u32) -> gtk::TextIter {
        unsafe {
            let mut iter = gtk::TextIter::uninitialized();
            gtk_source_sys::gtk_source_gutter_lines_get_iter_at_line(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, line);
            iter
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_last(&self) -> u32 {
        unsafe {
            gtk_source_sys::gtk_source_gutter_lines_get_last(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_line_yrange(&self, line: u32, mode: GutterRendererAlignmentMode) -> (i32, i32) {
        unsafe {
            let mut y = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            gtk_source_sys::gtk_source_gutter_lines_get_line_yrange(self.as_ref().to_glib_none().0, line, mode.to_glib(), y.as_mut_ptr(), height.as_mut_ptr());
            let y = y.assume_init();
            let height = height.assume_init();
            (y, height)
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_view(&self) -> Option<gtk::TextView> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_gutter_lines_get_view(self.as_ref().to_glib_none().0))
        }
    }

    fn get_yrange(&self, line: u32, line_y: u32, line_height: u32) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_lines_get_yrange(self.as_ref().to_glib_none().0, line, line_y, line_height);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn has_class(&self, line: u32, name: &str) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_gutter_lines_has_class(self.as_ref().to_glib_none().0, line, name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn has_qclass(&self, line: u32, qname: glib::Quark) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_gutter_lines_has_qclass(self.as_ref().to_glib_none().0, line, qname.to_glib()))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn is_cursor(&self, line: u32) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_gutter_lines_is_cursor(self.as_ref().to_glib_none().0, line))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn is_prelit(&self, line: u32) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_gutter_lines_is_prelit(self.as_ref().to_glib_none().0, line))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn is_selected(&self, line: u32) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_gutter_lines_is_selected(self.as_ref().to_glib_none().0, line))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn remove_class(&self, line: u32, name: &str) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_lines_remove_class(self.as_ref().to_glib_none().0, line, name.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn remove_qclass(&self, line: u32, qname: glib::Quark) {
        unsafe {
            gtk_source_sys::gtk_source_gutter_lines_remove_qclass(self.as_ref().to_glib_none().0, line, qname.to_glib());
        }
    }
}

impl fmt::Display for GutterLines {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GutterLines")
    }
}
