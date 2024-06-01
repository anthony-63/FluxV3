#pragma once

#include <vector>
#include <content/BeatmapSet.hpp>

class FluxClass {
public:
    std::vector<godot::BeatmapSet> loaded_beatmapsets = {};
};

static FluxClass Flux = FluxClass();