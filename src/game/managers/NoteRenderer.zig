const rl = @import("raylib");

const Note = @import("../objects/Note.zig");
const SyncManager = @import("SyncManager.zig");

NoteMesh: rl.Mesh,
NoteMaterial: rl.Material,

ApproachTime: f64,

ToRender: []Note,

SyncManager: SyncManager,

pub fn init(sync_manager: *SyncManager) !@This() {
    const renderer: @This() = .{
        .SyncManager = sync_manager,
        .ToRender = {},
        .NoteMaterial = rl.loadMaterialDefault(),
    };

    return renderer;
}
