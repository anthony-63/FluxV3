#pragma once

#include <stdint.h>

typedef struct {
    float x, y, time;
} notedata_t;

typedef struct {
    int broken;
    uint8_t version;

    const char* name;
    notedata_t* notes;
} difficulty_t;

difficulty_t difficulty_from_file(const char* path);