#pragma once

#include <stdint.h>

#include "difficulty.h"

typedef struct {
    int broken;
    uint8_t version;

    const char* path;
    const char* artist;
    const char* title;
    
    char** mappers;

    const uint8_t* cover;

    difficulty_t* difficulties;
} beatmap_t;

beatmap_t beatmap_from_folder(char* path);