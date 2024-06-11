const std = @import("std");

const BeatmapSet = @import("content/BeatmapSet.zig");
const Beatmap = @import("content/Beatmap.zig");

pub var GameFolder: []u8 = "";

pub var SelectedBeatmapSet: ?BeatmapSet = null;
pub var SelectedDiffIndex: u8 = 0;

pub fn deinit(allocator: std.mem.Allocator) void {
    allocator.free(GameFolder);
    if (SelectedBeatmapSet != null) {
        SelectedBeatmapSet.?.deinit(allocator);
    }
}
