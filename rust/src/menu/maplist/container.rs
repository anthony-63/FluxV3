use godot::{engine::{IVBoxContainer, VBoxContainer}, prelude::*};

use crate::FLUX;

use super::map_button::MapButton;

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct MapContainer {
    base: Base<VBoxContainer>,
}

#[godot_api]
impl IVBoxContainer for MapContainer {
    fn init(base: Base<VBoxContainer>) -> Self {
        Self {
            base,
        }
    }

    fn enter_tree(&mut self) {
        
        let entry_prefab = load::<PackedScene>("res://prefabs/map_button.tscn");
        unsafe {
            for map in FLUX.loaded_mapsets.clone() {
                for diff in map.difficulties.clone() {
                    let mut entry = entry_prefab.instantiate_as::<MapButton>();
                    entry.call("set_data".into(), &[Gd::from_object(diff).to_variant(), Gd::from_object(map.clone()).to_variant()]);
                    entry.set_visible(true);
                    self.base_mut().add_child(entry.upcast::<Node>());
                }
            }
        }
    }
}

#[godot_api]
impl MapContainer {
    
}