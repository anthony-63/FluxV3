#pragma once

#include "objects/grid.h"
#include "objects/camera.h"

typedef struct {
    grid_t grid;
    camera_t camera;
} game_t;

game_t* make_game();
void update_game(game_t* game);
void draw_game(game_t* game);