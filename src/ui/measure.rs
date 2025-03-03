use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Measure(ObjectSubclass<imp::MeasureImpl>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Measure {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for Measure {
    fn default() -> Self {
        Self::new()
    }
}

mod imp {
    use std::str::FromStr;

    use gtk::gdk::prelude::GdkCairoContextExt;
    use gtk::glib::subclass::*;
    use gtk::prelude::*;
    use gtk::{gdk, glib, CompositeTemplate};
    use gtk::subclass::prelude::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/jeremiahcrosby/tabplayer/measure.ui")]
    pub struct MeasureImpl {
        #[template_child]
        pub drawing_area: TemplateChild<gtk::DrawingArea>
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for MeasureImpl {
        const NAME: &'static str = "TabPlayerMeasure";
        type Type = super::Measure;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
    
        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for MeasureImpl {
        fn constructed(&self) {
            let rgba = gdk::RGBA::from_str("blue").unwrap();
            self.drawing_area.set_draw_func(move |_, cr, _width, _height| {
                cr.set_source_color(&rgba);
                cr.paint().expect("Invalid cairo surface state");
            });
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for MeasureImpl {
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            self.parent_size_allocate(width, height, baseline);
        }
    }
}