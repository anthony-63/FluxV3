use godot::{engine::{Button, Control, IControl, InputEvent, Label, TextureRect}, prelude::*};

use crate::{content::maploader::MapLoader, FLUX};
use rand::prelude::SliceRandom;

use super::{maplist::{details::MapDetails, Maplist}, menu::Menu, settings::SettingsMenu};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Viewports {
    base: Base<Control>,
    menu_view: Option<Gd<Menu>>,
    maplist_view: Option<Gd<Maplist>>,
    settings_view: Option<Gd<SettingsMenu>>,
    song_name: Option<Gd<Label>>,
}

#[godot_api]
impl IControl for Viewports {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            menu_view: None,
            maplist_view: None,
            settings_view: None,
            song_name: None,
        }
    }

    fn enter_tree(&mut self) {
        
        self.menu_view = Some(self.base().get_node_as::<Menu>("Viewports/Menu"));
        self.maplist_view = Some(self.base().get_node_as::<Maplist>("Viewports/Maplist"));
        self.settings_view = Some(self.base().get_node_as::<SettingsMenu>("Viewports/SettingsMenu"));
        self.song_name = Some(self.base().get_node_as::<Label>("TopPanel/MusicPlayer/SongName"));

        let mut menu_view = self.menu_view.clone().unwrap();
        let mut maplist_view = self.maplist_view.clone().unwrap();
        let mut settings_view = self.settings_view.clone().unwrap();

        menu_view.connect("change_to_maplist".into(), self.base_mut().callable("change_to_maplist"));
        menu_view.connect("open_settings".into(), self.base_mut().callable("toggle_settings"));

        maplist_view.connect("change_to_menu".into(), self.base_mut().callable("change_to_menu"));

        settings_view.connect("close_settings".into(), self.base_mut().callable("close_settings"));
        settings_view.set_visible(false);

        let maps_dragged_callable = self.base_mut().callable("maps_dragged");
        self.base_mut().get_viewport().unwrap().connect("files_dropped".into(), maps_dragged_callable);

        let mut pause_button: Gd<Button> = self.base().get_node_as::<Button>("TopPanel/MusicPlayer/Pause");
        pause_button.connect("pressed".into(), self.base_mut().callable("toggle_music"));
        let mut skip_button: Gd<Button> = self.base().get_node_as::<Button>("TopPanel/MusicPlayer/Skip");
        skip_button.connect("pressed".into(), self.base_mut().callable("skip_music"));

        let mut map_details = Some(self.base().get_node_as::<MapDetails>("Viewports/Maplist/MapDetails"));
        let mut bg_blur = Some(self.base().get_node_as::<TextureRect>("Viewports/Maplist/BgBlur"));
        
        if unsafe { FLUX.should_open_details && FLUX.selected_mapset.is_some() } {
            self.change_visibility(false, true);

            match map_details.as_mut().unwrap().try_call("set_details".into(), &[]) {
                Ok(_) => {},
                Err(e) => godot_print!("{}", e),
            }

            map_details.as_mut().unwrap().set_visible(true);
            bg_blur.as_mut().unwrap().set_visible(true);

            unsafe { FLUX.should_open_details = false; }
        } else {
            self.change_visibility(true, false);
        }
    }

    fn process(&mut self, _: f64) {
        unsafe {
            if FLUX.selected_mapset.is_some() {
                let title = FLUX.selected_mapset.clone().unwrap().bind().title.clone();
                self.song_name.as_mut().unwrap().set_text(title.into());
            } else {
                self.song_name.as_mut().unwrap().set_text("Not playing anything.".into());
            }
        }
        unsafe {
            if FLUX.loaded_mapsets.len() > 0 {
                let mut music = self.base().get_node_as::<AudioStreamPlayer>("Music");
                if !music.is_playing() && !music.get_stream_paused() {
                    let music_stream = FLUX.selected_mapset.clone().unwrap().bind().load_audio(true);
                    if music_stream.is_some() {
                        music.set_stream(music_stream.unwrap());
                        music.play();
                    }
                }
            }
        }

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
    fn maps_dragged(&mut self, maps: PackedStringArray) {
        for path_gstr in maps.as_slice() {
            let path = path_gstr.to_string();
            MapLoader::add_map(path);
        }
    }

    #[func]
    fn toggle_music(&mut self) {
        let mut music = self.base().get_node_as::<AudioStreamPlayer>("Music");
        if music.is_playing() {
            music.set_stream_paused(true);
        } else {
            music.set_stream_paused(false);
        }
    }

    #[func]
    fn skip_music(&mut self) {
        let mut music = self.base().get_node_as::<AudioStreamPlayer>("Music");

        unsafe { 
            FLUX.selected_mapset = Some(Gd::from_object(FLUX.loaded_mapsets.choose(&mut rand::thread_rng()).unwrap().clone()));
            FLUX.selected_map = Some(Gd::from_object(FLUX.selected_mapset.clone().unwrap().bind().difficulties.choose(&mut rand::thread_rng()).unwrap().clone()));
        }
        
        let music_stream = unsafe {  FLUX.selected_mapset.clone().unwrap().bind().load_audio(true) };
        if music_stream.is_some() {
            music.set_stream(music_stream.unwrap());
            music.play();
        }
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