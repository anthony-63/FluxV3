#include "grid.h"

grid_t make_grid() {
    grid_t grid = {
        .sprite = make_sprite(
            (Vector3){0, 0, 0,}, 
            (Vector3){90, 0, 0}, 
            (Vector2){6, 6}, 
            SPRITE_MESH_PLANE, 
            "C:/Users/antho/AppData/Roaming/Flux/skins/Default/grid.png"
        ),
    };
    return grid;
}

void draw_grid(grid_t grid) {
    draw_sprite(grid.sprite);
}