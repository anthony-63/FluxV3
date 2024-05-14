use godot::{engine::{Control, IControl}, prelude::*};

use super::{maplist::Maplist, menu::Menu};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Viewports {
    base: Base<Control>,
    menu_view: Option<Gd<Menu>>,
    maplist_view: Option<Gd<Maplist>>,
}

#[godot_api]
impl IControl for Viewports {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            menu_view: None,
            maplist_view: None,
        }
    }

    fn enter_tree(&mut self) {
        self.menu_view = Some(self.base().get_node_as::<Menu>("Viewports/Menu"));
        self.maplist_view = Some(self.base().get_node_as::<Maplist>("Viewports/Maplist"));

        let mut menu_view = self.menu_view.clone().unwrap();
        let mut maplist_view = self.maplist_view.clone().unwrap();

        menu_view.connect("change_to_maplist".into(), self.base_mut().callable("change_to_maplist"));
        maplist_view.connect("change_to_menu".into(), self.base_mut().callable("change_to_menu"));

        self.change_visibility(true, false);
    }
}

#[godot_api]
impl Viewports {
    #[func]
    fn change_to_maplist(&mut self) {
        self.change_visibility(false, true);
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