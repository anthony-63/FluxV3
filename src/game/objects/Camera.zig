const rl = @import("raylib");

const Settings = @import("../../Settings.zig");

Pitch: f32,
Yaw: f32,
RlCamera: rl.Camera3D,

pub fn init() !@This() {
    const position = rl.Vector3.init(0, 0, 7.5);

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

pub fn doParallax(self: *@This(), cursor_position: rl.Vector2) void {
    var pos = self.RlCamera.position;
    pos.x = @as(f32, @floatCast((cursor_position.x * Settings.Camera.Parallax) / 50.0));
    pos.y = -@as(f32, @floatCast((cursor_position.y * Settings.Camera.Parallax) / 50.0));
    self.RlCamera.position = pos;
    self.RlCamera.target = rl.Vector3.init(pos.x, pos.y, 0.0);
}
