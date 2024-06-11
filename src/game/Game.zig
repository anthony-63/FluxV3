const rl = @import("raylib");
const Camera = @import("objects/Camera.zig");

Camera: Camera,

pub fn init() !@This() {
    return .{ .Camera = try Camera.init(rl.Vector3.init(0, 0, 7.5)) };
}

pub fn draw(self: @This()) void {
    rl.clearBackground(rl.Color.black);
    self.Camera.RlCamera.begin();
    defer self.Camera.RlCamera.end();
}

pub fn update(self: @This()) void {
    _ = self;
}
