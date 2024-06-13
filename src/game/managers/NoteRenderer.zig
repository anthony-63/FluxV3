const std = @import("std");
const rl = @import("raylib");

const Settings = @import("../../Settings.zig");
const Global = @import("../../Global.zig");

const Note = @import("../objects/Note.zig");
const SyncManager = @import("SyncManager.zig");

NoteMesh: ?rl.Mesh,
NoteMaterial: rl.Material,

ApproachTime: f64,

ToRender: ?[]Note,

SyncManager: *SyncManager,

pub fn init(sync_manager: *SyncManager, mesh_path: []const u8, allocator: std.mem.Allocator) !@This() {
    var buf: [std.fs.max_path_bytes]u8 = undefined;
    const full_mesh_path = try allocator.dupeZ(u8, try Global.SkinsFolder.?.realpath(mesh_path, buf[0..]));
    defer allocator.free(full_mesh_path);

    var renderer: @This() = .{
        .SyncManager = sync_manager,
        .ToRender = undefined,
        .NoteMaterial = rl.loadMaterialDefault(),
        .ApproachTime = Settings.Note.ApproachTime,
        .NoteMesh = undefined,
    };

    const model = rl.loadModel(full_mesh_path);
    renderer.NoteMesh = model.meshes[0];
    renderer.NoteMaterial.maps[0].color = rl.Color.white;

    return renderer;
}
