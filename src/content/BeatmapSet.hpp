#pragma once

#include <godot_cpp/classes/ref_counted.hpp>
#include <content/Beatmap.hpp>

namespace godot {
    class BeatmapSet : public RefCounted {
        GDCLASS(BeatmapSet, RefCounted);

    protected:
        static void _bind_methods() {}
    
    public:
        bool broken;

        uint8_t version;
        
        std::string hash;
        std::string path;
        std::string music_path;

        std::string artist;
        std::string title;
        std::vector<std::string> mappers;

        std::vector<Beatmap> difficulties;

        bool has_cover;
        std::vector<uint8_t> cover_buffer;

        static BeatmapSet from_folder(std::string path);
    };
}