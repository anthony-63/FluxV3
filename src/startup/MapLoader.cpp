#include "MapLoader.hpp"

#include <Flux.hpp>

#include <godot_cpp/variant/utility_functions.hpp>
#include <filesystem>
#include <util/io.hpp>

using namespace godot;

void MapLoader::load_all_from_dir(std::string path, Startup *startup, void(*update)(std::string, int, int, Startup* startup)) {
    print("Loading maps from: ", path.c_str());

    if(!std::filesystem::exists(path))
        std::filesystem::create_directory(path);

    std::error_code ec;
    const std::filesystem::directory_iterator dirEnd;

    int total = std::distance(std::filesystem::directory_iterator(path), std::filesystem::directory_iterator{});

    int count = 0;
    // safe iterator
    for (auto &entry : std::filesystem::directory_iterator(path)) {
        if(!entry.is_directory()) continue;

        Flux.loaded_beatmapsets.push_back(BeatmapSet::from_folder(entry.path().u8string()));
        update(entry.path().stem().u8string(), count, total, startup);

        count++;
    }
}