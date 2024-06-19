#pragma once

#include "objects/grid.h"

typedef struct {
    grid_t grid;
} game_t;

game_t* make_game();
void update_game(game_t* game);
void draw_game(game_t* game);