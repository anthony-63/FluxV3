const std = @import("std");

const BeatmapSet = @import("content/BeatmapSet.zig");
const Beatmap = @import("content/Beatmap.zig");

pub const GameFolder = "%AppData%/Flux";

pub var SelectedBeatmapSet: ?BeatmapSet = null;
pub var SelectedDiffIndex: u8 = 0;

pub fn deinit(allocator: std.mem.Allocator) void {
    SelectedBeatmapSet.?.deinit(allocator);
}
