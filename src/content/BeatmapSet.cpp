#include "BeatmapSet.hpp"

#include <godot_cpp/variant/utility_functions.hpp>

#include <filesystem>
#include <fstream>
#include <sstream>

#include <util/io.hpp>
#include <util/rapidjson/document.h>

using namespace godot;

BeatmapSet BeatmapSet::from_folder(std::string path) {
    BeatmapSet set = BeatmapSet();

    std::string base_name = std::filesystem::path(path).stem().string();

    std::string meta_path = path + "/meta.json";

    CHECK_BROKEN(std::filesystem::exists(meta_path), set, std::string("meta json doesnt exist at path: " + path + "\nmarking as broken...").c_str());

    std::ifstream meta_istream(meta_path);
    std::stringstream meta_sstream;
    meta_sstream << meta_istream.rdbuf();
    
    rapidjson::Document meta;
    meta.Parse(meta_sstream.str().c_str());

    JSON_EXISTS_BEATMAP(meta, "_version", set);
    JSON_EXISTS_BEATMAP(meta, "_title", set);
    JSON_EXISTS_BEATMAP(meta, "_mappers", set);
    JSON_EXISTS_BEATMAP(meta, "_difficulties", set);
    JSON_EXISTS_BEATMAP(meta, "_music", set);
    
    CHECK_BROKEN(meta["_version"].IsInt(), set, "_version has wrong type, should be int");
    CHECK_BROKEN(meta["_title"].IsString(), set, "_title has wrong type, should be string");
    CHECK_BROKEN(meta["_music"].IsString(), set, "_music has wrong type, should be string");
    CHECK_BROKEN(meta["_mappers"].IsArray(), set, "_mappers has wrong type, should be array");
    CHECK_BROKEN(meta["_difficulties"].IsArray(), set, "_difficulties has wrong type, should be array");

    set.version = (uint8_t)meta["_version"].GetInt();
    set.title = meta["_title"].GetString();
    set.music_path = meta["_music"].GetString();
    
    for(auto &m : meta["_mappers"].GetArray()) {
        set.mappers.push_back(m.GetString());
    }

    for(auto &d : meta["_difficulties"].GetArray()) {
        std::string diff_path = path + "/" + d.GetString();
        set.difficulties.push_back(Beatmap::from_file(diff_path));
    }

    return set;
}