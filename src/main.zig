const std = @import("std");
const BeatmapSet = @import("content/BeatmapSet.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }

    BeatmapSet.loadFromFolder("C:/Users/antho/AppData/Roaming/Flux/maps/zitronitro_7_7_-_mii_munbe_plaza", allocator) catch |err| {
        std.log.err("Error loading beatmap: {any}", .{err});
    };
}
