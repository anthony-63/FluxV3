const rl = @import("raylib");

const Camera = @import("objects/Camera.zig");
const Grid = @import("objects/Grid.zig");

Camera: Camera,
Grid: Grid,

pub fn init() !@This() {
    return .{ .Camera = try Camera.init(rl.Vector3.init(0, 0, 7.5)), .Grid = try Grid.init("C:/Users/antho/AppData/Roaming/Flux/skins/Default/grid.png") };
}

pub fn draw(self: @This()) void {
    rl.clearBackground(rl.Color.black);
    self.Camera.RlCamera.begin();
    defer self.Camera.RlCamera.end();

    self.Grid.draw();
}

pub fn update(self: @This()) void {
    _ = self;
}
