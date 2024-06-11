const rl = @import("raylib");

Pitch: f32,
Yaw: f32,
RlCamera: rl.Camera3D,

pub fn init(position: rl.Vector3) !@This() {
    return .{
        .RlCamera = rl.Camera3D{
            .position = position,
            .target = rl.Vector3.init(0, 0, 0),
            .up = rl.Vector3.init(0, 1, 0),
            .fovy = 70,
            .projection = rl.CameraProjection.camera_perspective,
        },
        .Yaw = 0,
        .Pitch = 0,
    };
}
