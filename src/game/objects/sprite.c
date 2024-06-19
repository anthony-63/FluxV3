#include "sprite.h"

#include <rlgl.h>

sprite_t make_sprite(Vector3 position, Vector3 rotation, Vector2 size, sprite_mesh_t mesh_type, char* texture_path) {
    sprite_t sprite = {
        .position = position,
        .rotation = rotation,
        .size = size,
    };

    sprite.model = LoadModelFromMesh(GenMeshPlane(size.x, size.y, 1, 1));
    Image img = LoadImage(texture_path);
    Texture tex = LoadTextureFromImage(img);
    UnloadImage(img);

    sprite.model.materials[0].maps[MATERIAL_MAP_DIFFUSE].texture = tex;
    sprite.model.materials[0].maps[MATERIAL_MAP_DIFFUSE].color = WHITE;
    return sprite;
}

sprite_t draw_sprite(sprite_t sprite) {
    rlPushMatrix();
    rlRotatef(sprite.rotation.x, 1, 0, 0);
    rlRotatef(sprite.rotation.y, 0, 1, 0);
    rlRotatef(sprite.rotation.z, 0, 0, 1);
    DrawModel(sprite.model, sprite.position, 1, WHITE);
    rlPopMatrix();
}
