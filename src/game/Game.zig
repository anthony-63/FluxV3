const std = @import("std");
const rl = @import("raylib");

const Global = @import("../Global.zig");
const Camera = @import("objects/Camera.zig");
const Grid = @import("objects/Grid.zig");

const SyncManager = @import("managers/SyncManager.zig");
const NoteRenderer = @import("managers/NoteRenderer.zig");
const NoteManager = @import("managers/NoteManager.zig");

Camera: Camera,
Grid: Grid,

SyncManager: SyncManager,
NoteRenderer: NoteRenderer,
NoteManager: NoteManager,

Playing: bool,

Allocator: std.mem.Allocator,

pub fn init(allocator: std.mem.Allocator) !@This() {
    return .{
        .Camera = try Camera.init(rl.Vector3.init(0, 0, 7.5)),
        .Grid = try Grid.init("Default/grid.png", allocator),
        .SyncManager = try SyncManager.init(allocator),
        .NoteRenderer = try NoteRenderer.init("Default/mesh.obj", allocator),
        .NoteManager = try NoteManager.init(allocator),
        .Playing = false,

        .Allocator = allocator,
    };
}

pub fn draw(self: *@This()) void {
    rl.clearBackground(rl.Color.black);
    rl.drawFPS(0, 0);

    self.Camera.RlCamera.begin();
    defer self.Camera.RlCamera.end();

    self.NoteRenderer.drawSingle(self.SyncManager);
    self.Grid.draw();
}

pub fn update(self: *@This()) !void {
    if (!self.Playing) {
        self.SyncManager.start(0);
        self.Playing = true;
    }

    self.SyncManager.update();
    try self.NoteManager.update(&self.NoteRenderer, self.SyncManager);
}

pub fn deinit(self: @This()) void {
    self.SyncManager.deinit(self.Allocator);
    self.NoteManager.deinit(self.Allocator);
    self.NoteRenderer.deinit();
}
