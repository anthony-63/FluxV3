#include "game.h"

#include <stdlib.h>

game_t* make_game() {
    game_t* game = (game_t*)malloc(sizeof(game_t));
    game->grid = make_grid();
    game->camera = make_camera();
    return game;
}

void update_game(game_t* game) {

}

void draw_game(game_t* game) {
    start_camera(&game->camera);
    draw_grid(game->grid);
    end_camera(&game->camera);
}
