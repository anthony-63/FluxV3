#pragma once

#include <raylib.h>

typedef enum {
    SPRITE_MESH_PLANE,
} sprite_mesh_t;

typedef struct {
    Vector3 position, rotation;
    Vector2 size;

    Model model;
} sprite_t;

sprite_t make_sprite(Vector3 position, Vector3 rotation, Vector2 size, sprite_mesh_t mesh_type, char* texture_path);
sprite_t draw_sprite(sprite_t sprite);
