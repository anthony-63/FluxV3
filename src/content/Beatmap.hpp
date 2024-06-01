#pragma once

#include <godot_cpp/classes/ref_counted.hpp>

#define CHECK_BROKEN(b, map, error) \
    if(!b) { \
        print_err(error); \
        map.broken = true; \
        return map; \
    }

#define JSON_EXISTS_BEATMAP(doc, name, map) CHECK_BROKEN(doc.HasMember(name), map, "map json doesnt have member " #name "\nsetting broken...")

typedef struct {
    double x;
    double y;
    double t;
} NoteData;

namespace godot {
    class Beatmap : public RefCounted {
        GDCLASS(Beatmap, RefCounted);

    protected:
        static void _bind_methods() {}

    public:
        bool broken;

        uint8_t version;

        std::string path;
        std::string name;
        std::string id;

        std::vector<NoteData> notes;

        static Beatmap from_file(std::string path);
    };
}