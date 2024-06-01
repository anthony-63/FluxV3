#pragma once

#include <startup/Startup.hpp>
#include <string>

namespace godot {
    class MapLoader {
        public:
            static void load_all_from_dir(std::string path, Startup *startup, void(*update)(std::string, int, int, Startup* startup));
    };
}