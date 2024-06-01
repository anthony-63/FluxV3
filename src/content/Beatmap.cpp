#include "Beatmap.hpp"

#include <godot_cpp/variant/utility_functions.hpp>

#include <filesystem>
#include <fstream>
#include <sstream>

#include <util/io.hpp>
#include <util/rapidjson/document.h>

using namespace godot;

Beatmap Beatmap::from_file(std::string path) {
    Beatmap map = Beatmap();

    CHECK_BROKEN(std::filesystem::exists(path), map, std::string("diff json doesnt exist at path: " + path + "\nmarking as broken...").c_str());

    std::ifstream data_istream(path);
    std::stringstream data_sstream;
    data_sstream << data_istream.rdbuf();
    
    rapidjson::Document data;
    data.Parse(data_sstream.str().c_str());

    JSON_EXISTS_BEATMAP(data, "_version", map);
    JSON_EXISTS_BEATMAP(data, "_name", map);
    JSON_EXISTS_BEATMAP(data, "_notes", map);
    
    CHECK_BROKEN(data["_version"].IsInt(), map, "_version has wrong type, should be int");
    CHECK_BROKEN(data["_name"].IsString(), map, "_name has wrong type, should be string");
    CHECK_BROKEN(data["_notes"].IsArray(), map, "_notes has wrong type, should be array");

    map.version = (uint8_t)data["_version"].GetInt();
    map.name = data["_name"].GetString();
    
    int count = 0;
    for(auto &n : data["_notes"].GetArray()) {
        CHECK_BROKEN(n.IsObject(), map, "note has wrong type, should be object");
        
        NoteData note = NoteData();

        JSON_EXISTS_BEATMAP(n, "_x", map);
        JSON_EXISTS_BEATMAP(n, "_y", map);
        JSON_EXISTS_BEATMAP(n, "_time", map);

        if(!n["_x"].IsDouble() && !n["_x"].IsInt()) CHECK_BROKEN(false, map, "_x has wrong type, should be int || double");
        if(!n["_y"].IsDouble() && !n["_y"].IsInt()) CHECK_BROKEN(false, map, "_y has wrong type, should be int || double");
        if(!n["_time"].IsDouble() && !n["_time"].IsInt()) CHECK_BROKEN(false, map, "_time has wrong type, should be int || double");

        if(n["_time"].IsDouble()) note.t = n["_time"].GetDouble();
        else note.t = n["_time"].GetInt();

        if(n["_x"].IsDouble()) note.x = n["_x"].GetDouble();
        else note.x = n["_x"].GetInt();

        if(n["_y"].IsDouble()) note.y = n["_y"].GetDouble();
        else note.y = n["_y"].GetInt();

        count++;
    }

    return map;
}
