mod imp {
    use gtk4::{
        gio::MenuModel, glib, prelude::*, subclass::prelude::*, CompositeTemplate, MenuButton,
        PopoverMenu, ToggleButton,
    };

    #[derive(Default, Debug, CompositeTemplate)]
    #[template(resource = "/com/github/flxzt/rnote/ui/appmenu.ui")]
    pub struct AppMenu {
        #[template_child]
        pub menubutton: TemplateChild<MenuButton>,
        #[template_child]
        pub popovermenu: TemplateChild<PopoverMenu>,
        #[template_child]
        pub menu_model: TemplateChild<MenuModel>,
        #[template_child]
        pub default_theme_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub light_theme_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub dark_theme_toggle: TemplateChild<ToggleButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppMenu {
        const NAME: &'static str = "AppMenu";
        type Type = super::AppMenu;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AppMenu {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            self.menubutton
                .get()
                .set_popover(Some(&self.popovermenu.get()));
        }

        fn dispose(&self, obj: &Self::Type) {
            while let Some(child) = obj.first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for AppMenu {
        fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
            self.parent_size_allocate(widget, width, height, baseline);
            self.popovermenu.get().present();
        }
    }
}

use crate::ui::appwindow::RnoteAppWindow;
use gtk4::{
    gio, glib, glib::clone, prelude::*, subclass::prelude::*, MenuButton, PopoverMenu,
    ToggleButton, Widget,
};

glib::wrapper! {
    pub struct AppMenu(ObjectSubclass<imp::AppMenu>)
    @extends Widget;
}

impl Default for AppMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl AppMenu {
    pub fn new() -> Self {
        let appmenu: AppMenu = glib::Object::new(&[]).expect("Failed to create AppMenu");
        appmenu
    }

    pub fn menubutton(&self) -> MenuButton {
        imp::AppMenu::from_instance(self).menubutton.get()
    }

    pub fn popovermenu(&self) -> PopoverMenu {
        imp::AppMenu::from_instance(self).popovermenu.get()
    }

    pub fn menu_model(&self) -> gio::MenuModel {
        imp::AppMenu::from_instance(self).menu_model.get()
    }

    pub fn default_theme_toggle(&self) -> ToggleButton {
        imp::AppMenu::from_instance(self).default_theme_toggle.get()
    }

    pub fn light_theme_toggle(&self) -> ToggleButton {
        imp::AppMenu::from_instance(self).light_theme_toggle.get()
    }

    pub fn dark_theme_toggle(&self) -> ToggleButton {
        imp::AppMenu::from_instance(self).dark_theme_toggle.get()
    }

    pub fn init(&self, appwindow: &RnoteAppWindow) {
        self.default_theme_toggle().connect_toggled(
            clone!(@weak appwindow => move |default_theme_toggle| {
                if default_theme_toggle.is_active() {
                    appwindow.set_color_scheme(adw::ColorScheme::Default);
                }
            }),
        );

        self.light_theme_toggle().connect_toggled(
            clone!(@weak appwindow => move |light_theme_toggle| {
                if light_theme_toggle.is_active() {
                    appwindow.set_color_scheme(adw::ColorScheme::ForceLight);
                }
            }),
        );

        self.dark_theme_toggle().connect_active_notify(
            clone!(@weak appwindow => move |dark_theme_toggle| {
                if dark_theme_toggle.is_active() {
                    appwindow.set_color_scheme(adw::ColorScheme::ForceDark);
                }
            }),
        );
    }
}
