#pragma once

#include "sprite.h"

typedef struct {
    sprite_t sprite;
} grid_t;

grid_t make_grid();
void draw_grid(grid_t grid);