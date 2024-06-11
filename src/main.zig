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

    const env_map = try allocator.create(std.process.EnvMap);
    env_map.* = try std.process.getEnvMap(allocator);
    defer env_map.deinit();

    const appdata = env_map.get("AppData") orelse ".";
    Global.GameFolder = try std.fmt.allocPrint(allocator, "{s}/Flux", .{appdata});

    var window = try Window.init(allocator);
    window.run() catch |err| {
        std.log.err("Failed to run step game: {any}", .{err});
    };
    defer window.deinit();
}
