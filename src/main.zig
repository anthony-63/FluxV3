const std = @import("std");

const Window = @import("window/Window.zig");
const BeatmapSet = @import("content/BeatmapSet.zig");
const Global = @import("Global.zig");

pub fn getOrCreateAbs(path: []const u8, fail_message: []const u8) !std.fs.Dir {
    return std.fs.openDirAbsolute(path, .{}) catch |err| {
        switch (err) {
            error.FileNotFound => {
                try std.fs.makeDirAbsolute(path);
                return try std.fs.openDirAbsolute(path, .{});
            },
            else => {
                @panic(fail_message);
            },
        }
    };
}

pub fn getOrCreateChild(parent: std.fs.Dir, path: []const u8, fail_message: []const u8) !std.fs.Dir {
    return parent.openDir(path, .{}) catch |err| {
        switch (err) {
            error.FileNotFound => {
                parent.makeDir(path) catch {};
                return try parent.openDir(path, .{});
            },
            else => {
                @panic(fail_message);
            },
        }
    };
}

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
    const game_folder_path = try std.fmt.allocPrint(allocator, "{s}/Flux", .{appdata});
    Global.GameFolder = try getOrCreateAbs(game_folder_path, "Failed to create game folder. Should never happen.");
    allocator.free(game_folder_path);

    Global.MapsFolder = try getOrCreateChild(Global.GameFolder.?, "maps", "Failed to create maps folder. Should never happen.");
    Global.SkinsFolder = try getOrCreateChild(Global.GameFolder.?, "skins", "Failed to create skins folder. Should never happen?");

    var window = try Window.init(allocator);
    window.run() catch |err| {
        std.log.err("Failed to run step game: {any}", .{err});
    };
    defer window.deinit();
}
