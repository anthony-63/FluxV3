const std = @import("std");
const rl = @import("raylib");

const Global = @import("../Global.zig");
const Camera = @import("objects/Camera.zig");
const Grid = @import("objects/Grid.zig");
const SyncManager = @import("managers/SyncManager.zig");

Camera: Camera,
Grid: Grid,
SyncManager: SyncManager,

Playing: bool,

Allocator: std.mem.Allocator,

pub fn init(allocator: std.mem.Allocator) !@This() {
    return .{
        .Camera = try Camera.init(rl.Vector3.init(0, 0, 7.5)),
        .Grid = try Grid.init("./.game/skin/Default/grid.png"),
        .SyncManager = try SyncManager.init(allocator),
        .Playing = false,
        .Allocator = allocator,
    };
}

pub fn draw(self: @This()) void {
    rl.clearBackground(rl.Color.black);
    rl.drawFPS(0, 0);
    self.Camera.RlCamera.begin();
    defer self.Camera.RlCamera.end();

    self.Grid.draw();
}

pub fn update(self: *@This()) void {
    if (!self.Playing) {
        self.SyncManager.start(0);
        self.Playing = true;
    }

    self.SyncManager.update();
}

pub fn deinit(self: @This()) void {
    self.SyncManager.deinit(self.Allocator);
}
