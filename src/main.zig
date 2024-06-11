const std = @import("std");

const Window = @import("window/Window.zig");
const BeatmapSet = @import("content/BeatmapSet.zig");
const Global = @import("Global.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }

    var window = try Window.init(allocator);
    try window.run();
    defer window.deinit();
}
