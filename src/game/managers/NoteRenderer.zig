const std = @import("std");
const rl = @import("raylib");

const Settings = @import("../../Settings.zig");
const Global = @import("../../Global.zig");

const Note = @import("../objects/Note.zig");
const SyncManager = @import("SyncManager.zig");

NoteMesh: ?rl.Mesh,
NoteMaterial: rl.Material,

ApproachTime: f64,

ToRender: std.ArrayList(Note),

pub fn init(mesh_path: []const u8, allocator: std.mem.Allocator) !@This() {
    var buf: [std.fs.max_path_bytes]u8 = undefined;
    const full_mesh_path = try allocator.dupeZ(u8, try Global.SkinsFolder.?.realpath(mesh_path, buf[0..]));
    defer allocator.free(full_mesh_path);

    var renderer: @This() = .{
        .ToRender = std.ArrayList(Note).init(allocator),
        .NoteMaterial = rl.loadMaterialDefault(),
        .ApproachTime = Settings.Note.ApproachTime,
        .NoteMesh = undefined,
    };

    const model = rl.loadModel(full_mesh_path);
    renderer.NoteMesh = model.meshes[0];
    renderer.NoteMaterial.maps[0].color = rl.Color.white;

    return renderer;
}

pub fn drawSingle(self: @This(), sync: SyncManager) void {
    if (self.ToRender.items.len < 1) return;

    for (self.ToRender.items) |n| {
        const note_time = n.calculateTime(sync.RealTime, self.ApproachTime);
        const note_distance = note_time * Settings.Note.ApproachDistance;

        var transform = rl.Matrix.identity();
        transform = transform.multiply(rl.Matrix.translate(
            @floatCast(n.X * 2),
            @floatCast(n.Y * 2),
            @floatCast(-note_distance),
        ));

        var colored_mat = self.NoteMaterial;
        colored_mat.maps[0].color = n.Color;
        self.NoteMesh.?.draw(colored_mat, transform);
    }
}

pub fn deinit(self: @This()) void {
    self.ToRender.deinit();
}
