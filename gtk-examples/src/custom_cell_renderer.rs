use std::mem;
use std::ptr;
use std::sync::{Once, ONCE_INIT};

use glib_ffi;
use gobject_ffi;
use gtk_ffi;

use cairo;
use glib;
use glib::prelude::*;
use glib::translate::*;
use gtk;
use gtk::prelude::*;

mod imp {
    use super::*;

    use std::cell::RefCell;

    use gobject_subclass::object::*;

    use gtk_subclass::cell_renderer::*;

    pub struct CellRendererCustom {
        text: RefCell<String>,
    }

    static PROPERTIES: [Property; 1] = [Property::String(
        "text",
        "Text",
        "Text to render",
        None,
        PropertyMutability::ReadWrite,
    )];

    impl CellRendererCustom {
        pub fn get_type() -> glib::Type {
            static ONCE: Once = ONCE_INIT;
            static mut TYPE: glib::Type = glib::Type::Invalid;

            ONCE.call_once(|| {
                let static_instance = CellRendererCustomStatic;
                let t = register_type(static_instance);
                unsafe {
                    TYPE = t;
                }
            });

            unsafe { TYPE }
        }

        fn class_init(klass: &mut CellRendererClass) {
            klass.install_properties(&PROPERTIES);
        }

        fn init(_renderer: &CellRenderer) -> Box<CellRendererImpl<CellRenderer>> {
            let imp = Self {
                text: RefCell::new("".to_string()),
            };
            Box::new(imp)
        }

        // Useless dummy function to show how to expose APIs
        pub fn set_text(&self, text: &str) {
            self.text.replace(String::from(text));
        }
    }

    impl ObjectImpl<CellRenderer> for CellRendererCustom {
        fn set_property(&self, _obj: &glib::Object, id: u32, value: &glib::Value) {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("text", ..) => {
                    let text: String = value.get().unwrap();
                    self.text.replace(text);
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("text", ..) => Ok(self.text.borrow().clone().to_value()),
                _ => unimplemented!(),
            }
        }
    }

    impl CellRendererImpl<CellRenderer> for CellRendererCustom {
        fn render(
            &self,
            renderer: &CellRenderer,
            cr: &cairo::Context,
            widget: &gtk::Widget,
            _background_area: &gtk::Rectangle,
            cell_area: &gtk::Rectangle,
            _flags: gtk::CellRendererState,
        ) {
            let layout = widget
                .create_pango_layout(self.text.borrow().as_str())
                .unwrap();
            let sc = widget.get_style_context();
            let (padx, pady) = renderer.get_padding();

            cr.save();
            cr.rectangle(
                cell_area.x.into(),
                cell_area.y.into(),
                cell_area.width.into(),
                cell_area.height.into(),
            );
            cr.clip();

            gtk::render_layout(
                &sc,
                cr,
                (cell_area.x + padx).into(),
                (cell_area.y + pady).into(),
                &layout,
            );

            cr.restore();
        }
    }

    struct CellRendererCustomStatic;

    impl ImplTypeStatic<CellRenderer> for CellRendererCustomStatic {
        fn get_name(&self) -> &str {
            "CellRendererCustom"
        }

        fn new(&self, renderer: &CellRenderer) -> Box<CellRendererImpl<CellRenderer>> {
            CellRendererCustom::init(renderer)
        }

        fn class_init(&self, klass: &mut CellRendererClass) {
            CellRendererCustom::class_init(klass);
        }
    }
}

use gtk_subclass::cell_renderer;
use gobject_subclass::object::*;
use std::ops;

glib_wrapper! {
    pub struct CellRendererCustom(Object<imp::CellRendererCustom>):
        [cell_renderer::CellRenderer => InstanceStruct<cell_renderer::CellRenderer>,
         gtk::CellRenderer => gtk_ffi::GtkCellRenderer];

    match fn {
        get_type => || imp::CellRendererCustom::get_type().to_glib(),
    }
}

impl CellRendererCustom {
    pub fn new() -> CellRendererCustom {
        use glib::object::Downcast;

        unsafe {
            glib::Object::new(Self::static_type(), &[])
                .unwrap()
                .downcast_unchecked()
        }
    }
}

// TODO: This one should probably get a macro
impl ops::Deref for CellRendererCustom {
    type Target = imp::CellRendererCustom;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let base: cell_renderer::CellRenderer = from_glib_borrow(self.to_glib_none().0);
            let imp = base.get_impl();
            let imp = imp.downcast_ref::<imp::CellRendererCustom>().unwrap();
            // Cast to a raw pointer to get us an appropriate lifetime: the compiler
            // can't know that the lifetime of base is the same as the one of self
            &*(imp as *const imp::CellRendererCustom)
        }
    }
}
