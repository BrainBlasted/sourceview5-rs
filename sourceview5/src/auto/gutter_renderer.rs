// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::Buffer;
use crate::GutterLines;
use crate::GutterRendererAlignmentMode;
use crate::View;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use std::mem;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct GutterRenderer(Object<ffi::GtkSourceGutterRenderer, ffi::GtkSourceGutterRendererClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        get_type => || ffi::gtk_source_gutter_renderer_get_type(),
    }
}

pub const NONE_GUTTER_RENDERER: Option<&GutterRenderer> = None;

pub trait GutterRendererExt: 'static {
    fn activate(
        &self,
        iter: &gtk::TextIter,
        area: &gdk::Rectangle,
        button: u32,
        state: gdk::ModifierType,
        n_presses: i32,
    );

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn align_cell(&self, line: u32, width: f32, height: f32) -> (f32, f32);

    fn get_alignment_mode(&self) -> GutterRendererAlignmentMode;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_buffer(&self) -> Option<Buffer>;

    fn get_view(&self) -> Option<View>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_xalign(&self) -> f32;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_xpad(&self) -> i32;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_yalign(&self) -> f32;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_ypad(&self) -> i32;

    fn query_activatable(&self, iter: &gtk::TextIter, area: &gdk::Rectangle) -> bool;

    fn set_alignment_mode(&self, mode: GutterRendererAlignmentMode);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_xalign(&self, xalign: f32);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_xpad(&self, xpad: i32);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_yalign(&self, yalign: f32);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_ypad(&self, ypad: i32);

    fn get_property_lines(&self) -> Option<GutterLines>;

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn get_property_xpad(&self) -> i32;

    fn set_property_xpad(&self, xpad: i32);

    fn get_property_yalign(&self) -> f32;

    fn set_property_yalign(&self, yalign: f32);

    fn get_property_ypad(&self) -> i32;

    fn set_property_ypad(&self, ypad: i32);

    fn connect_activate<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, u32, gdk::ModifierType, i32) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_query_activatable<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    //fn connect_query_data<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alignment_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_lines_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRenderer>> GutterRendererExt for O {
    fn activate(
        &self,
        iter: &gtk::TextIter,
        area: &gdk::Rectangle,
        button: u32,
        state: gdk::ModifierType,
        n_presses: i32,
    ) {
        unsafe {
            ffi::gtk_source_gutter_renderer_activate(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                area.to_glib_none().0,
                button,
                state.to_glib(),
                n_presses,
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn align_cell(&self, line: u32, width: f32, height: f32) -> (f32, f32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::gtk_source_gutter_renderer_align_cell(
                self.as_ref().to_glib_none().0,
                line,
                width,
                height,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (x, y)
        }
    }

    fn get_alignment_mode(&self) -> GutterRendererAlignmentMode {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_get_alignment_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_get_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_view(&self) -> Option<View> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_get_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_xalign(&self) -> f32 {
        unsafe { ffi::gtk_source_gutter_renderer_get_xalign(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_xpad(&self) -> i32 {
        unsafe { ffi::gtk_source_gutter_renderer_get_xpad(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_yalign(&self) -> f32 {
        unsafe { ffi::gtk_source_gutter_renderer_get_yalign(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_ypad(&self) -> i32 {
        unsafe { ffi::gtk_source_gutter_renderer_get_ypad(self.as_ref().to_glib_none().0) }
    }

    fn query_activatable(&self, iter: &gtk::TextIter, area: &gdk::Rectangle) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_query_activatable(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                area.to_glib_none().0,
            ))
        }
    }

    fn set_alignment_mode(&self, mode: GutterRendererAlignmentMode) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_alignment_mode(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_xalign(&self, xalign: f32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_xalign(self.as_ref().to_glib_none().0, xalign);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_xpad(&self, xpad: i32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_xpad(self.as_ref().to_glib_none().0, xpad);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_yalign(&self, yalign: f32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_yalign(self.as_ref().to_glib_none().0, yalign);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_ypad(&self, ypad: i32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_ypad(self.as_ref().to_glib_none().0, ypad);
        }
    }

    fn get_property_lines(&self) -> Option<GutterLines> {
        unsafe {
            let mut value = Value::from_type(<GutterLines as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"lines\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `lines` getter")
        }
    }

    fn get_property_xalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"xalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `xalign` getter")
                .unwrap()
        }
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"xalign\0".as_ptr() as *const _,
                Value::from(&xalign).to_glib_none().0,
            );
        }
    }

    fn get_property_xpad(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"xpad\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `xpad` getter")
                .unwrap()
        }
    }

    fn set_property_xpad(&self, xpad: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"xpad\0".as_ptr() as *const _,
                Value::from(&xpad).to_glib_none().0,
            );
        }
    }

    fn get_property_yalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"yalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `yalign` getter")
                .unwrap()
        }
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"yalign\0".as_ptr() as *const _,
                Value::from(&yalign).to_glib_none().0,
            );
        }
    }

    fn get_property_ypad(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"ypad\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ypad` getter")
                .unwrap()
        }
    }

    fn set_property_ypad(&self, ypad: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"ypad\0".as_ptr() as *const _,
                Value::from(&ypad).to_glib_none().0,
            );
        }
    }

    fn connect_activate<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, u32, gdk::ModifierType, i32) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<
            P,
            F: Fn(&P, &gtk::TextIter, &gdk::Rectangle, u32, gdk::ModifierType, i32) + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            iter: *mut gtk::ffi::GtkTextIter,
            area: *mut gdk::ffi::GdkRectangle,
            button: libc::c_uint,
            state: gdk::ffi::GdkModifierType,
            n_presses: libc::c_int,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GutterRenderer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(iter),
                &from_glib_borrow(area),
                button,
                from_glib(state),
                n_presses,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_query_activatable<
        F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_activatable_trampoline<
            P,
            F: Fn(&P, &gtk::TextIter, &gdk::Rectangle) -> bool + 'static,
        >(
            this: *mut ffi::GtkSourceGutterRenderer,
            iter: *mut gtk::ffi::GtkTextIter,
            area: *mut gdk::ffi::GdkRectangle,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean
        where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GutterRenderer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(iter),
                &from_glib_borrow(area),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    query_activatable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_query_data<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored object: GObject.Object
    //}

    fn connect_property_alignment_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_alignment_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alignment-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alignment_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_lines_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lines_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::lines\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_lines_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_view_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xalign_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xalign_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xpad_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xpad\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xpad_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_yalign_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::yalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_yalign_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ypad_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRenderer>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ypad\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ypad_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GutterRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GutterRenderer")
    }
}