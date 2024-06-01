#include "Startup.hpp"

#include <godot_cpp/variant/utility_functions.hpp>
#include <godot_cpp/classes/scene_tree.hpp>
#include <godot_cpp/classes/os.hpp>

#include <thread>

#include <startup/MapLoader.hpp>
#include <util/io.hpp>

using namespace godot;

void Startup::_bind_methods() {}

void load_maps_threaded(Startup* startup) {
    auto update_map_stage = [](std::string txt, int at, int total, Startup* startup) {
        startup->progress->call_deferred("set_value", at);
        startup->progress->call_deferred("set_max", total);
        startup->substage_label->call_deferred("set_text", txt.c_str());
    };

    std::string user_dir = std::string(OS::get_singleton()->get_user_data_dir().ascii().ptr());
    MapLoader::load_all_from_dir(user_dir + "/maps", startup, update_map_stage);
}

void start_load(Startup* startup) {
    startup->stage_label->call_deferred("set_text", "Loading maps");

    std::thread map_thread = std::thread(&load_maps_threaded, startup);
    map_thread.join();

    startup->get_tree()->call_deferred("change_scene_to_file", "res://scenes/menu.tscn");
}

void Startup::_ready() {
    progress = get_node<ProgressBar>(NodePath("../Progress"));
    substage_label = get_node<Label>(NodePath("../SubStage"));
    stage_label = get_node<Label>(NodePath("../Stage"));

    loading_thread = std::thread(&start_load, this);
}

Startup::Startup() {}
Startup::~Startup() {
    loading_thread.join();
}