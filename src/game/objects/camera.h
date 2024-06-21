#pragma once

#include <raylib.h>

typedef struct {
    float pitch, yaw;
    Camera3D rl_camera;
} camera_t;

camera_t make_camera();

void start_camera(camera_t* camera);
void end_camera(camera_t* camera);

void do_parallax_camera(camera_t* camera);