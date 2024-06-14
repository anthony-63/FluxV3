const std = @import("std");

const BeatmapSet = @import("content/BeatmapSet.zig");
const Beatmap = @import("content/Beatmap.zig");

pub var GameFolder: ?std.fs.Dir = null;
pub var MapsFolder: ?std.fs.Dir = null;
pub var SkinsFolder: ?std.fs.Dir = null;

pub var SelectedBeatmapSet: ?BeatmapSet = null;
pub var SelectedDiffIndex: u8 = 0;

// TEMPORARY
pub var StartFrom: f64 = 0;

pub fn deinit(allocator: std.mem.Allocator) void {
    GameFolder.?.close();
    MapsFolder.?.close();
    SkinsFolder.?.close();
    if (SelectedBeatmapSet != null) {
        SelectedBeatmapSet.?.deinit(allocator);
    }
}
