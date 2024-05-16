use godot::{engine::{Control, IControl, InputEvent}, prelude::*};

use super::{maplist::Maplist, menu::Menu, settings::SettingsMenu};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Viewports {
    base: Base<Control>,
    menu_view: Option<Gd<Menu>>,
    maplist_view: Option<Gd<Maplist>>,
    settings_view: Option<Gd<SettingsMenu>>,
}

#[godot_api]
impl IControl for Viewports {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            menu_view: None,
            maplist_view: None,
            settings_view: None,
        }
    }

    fn enter_tree(&mut self) {
        self.menu_view = Some(self.base().get_node_as::<Menu>("Viewports/Menu"));
        self.maplist_view = Some(self.base().get_node_as::<Maplist>("Viewports/Maplist"));
        self.settings_view = Some(self.base().get_node_as::<SettingsMenu>("Viewports/SettingsMenu"));

        let mut menu_view = self.menu_view.clone().unwrap();
        let mut maplist_view = self.maplist_view.clone().unwrap();
        let mut settings_view = self.settings_view.clone().unwrap();

        menu_view.connect("change_to_maplist".into(), self.base_mut().callable("change_to_maplist"));
        menu_view.connect("open_settings".into(), self.base_mut().callable("toggle_settings"));
        
        maplist_view.connect("change_to_menu".into(), self.base_mut().callable("change_to_menu"));

        settings_view.connect("close_settings".into(), self.base_mut().callable("close_settings"));
        settings_view.set_visible(false);

        self.change_visibility(true, false);
    }

    fn input(&mut self, _: Gd<InputEvent>) {
        let input = Input::singleton();

        if input.is_action_just_pressed("open_settings".into()) {
            self.toggle_settings();
        }
    }

}

#[godot_api]
impl Viewports {
    #[func]
    fn change_to_maplist(&mut self) {
        self.change_visibility(false, true);
    }

    #[func]
    fn toggle_settings(&mut self) {
        let mut settings_view = self.settings_view.clone().unwrap();
        let is_visible = settings_view.is_visible();
        settings_view.set_visible(!is_visible);
    }

    #[func]
    fn close_settings(&mut self) {
        let mut settings_view = self.settings_view.clone().unwrap();
        settings_view.set_visible(false);
    }

    #[func]
    fn change_to_menu(&mut self) {
        self.change_visibility(true, false);
    }

    fn change_visibility(&mut self, menu: bool, maplist: bool) {
        let mut menu_view = self.menu_view.clone().unwrap();
        let mut maplist_view = self.maplist_view.clone().unwrap();

        menu_view.set_visible(menu);
        maplist_view.set_visible(maplist);
    }
}