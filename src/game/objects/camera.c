#include "camera.h"

camera_t make_camera() {
    return (camera_t) {
        .rl_camera = (Camera3D) {
            .position = (Vector3){0, 0, 7.5},
            .target = (Vector3){0, 0, 0},
            .fovy = 70,

            .up = (Vector3){0, 1, 0},
            .projection = CAMERA_PERSPECTIVE,
        }
    };
}

void start_camera(camera_t* camera) {
    BeginMode3D(camera->rl_camera);
}

void end_camera(camera_t* camera) {
    EndMode3D();
}

void do_parallax_camera(camera_t* camera) {

}
