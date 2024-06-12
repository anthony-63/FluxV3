const std = @import("std");

const Window = @import("window/Window.zig");
const BeatmapSet = @import("content/BeatmapSet.zig");
const Global = @import("Global.zig");
const FPCK1 = @import("Content/FPCK1.zig");

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

fn runFpck1Store(allocator: std.mem.Allocator) void {
    var game_folder = std.fs.cwd().openDir("./.game", .{ .access_sub_paths = true, .iterate = true }) catch |err| {
        switch (err) {
            error.FileNotFound => {
                std.log.err(".game folder not present. Must exist to compress", .{});
                std.process.exit(1);
            },
            else => {
                std.log.err("Failed to open .game folder.", .{});
                std.process.exit(1);
            },
        }
    };
    defer game_folder.close();

    var out_file = std.fs.cwd().createFile("gamedata.fpck1", .{ .truncate = true }) catch {
        std.log.err("Failed to open/create gamedata.fpck1 file.", .{});
        std.process.exit(1);
    };
    defer out_file.close();

    FPCK1.pack(game_folder, out_file, allocator) catch |err| {
        std.log.err("Failed to compress game folder: {any}", .{err});
    };
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    for (args, 0..) |mode, i| {
        if (i != 1) continue;
        if (std.mem.eql(u8, mode, "fpck1")) {
            runFpck1Store(allocator);
        }
        return;
    } else {}

    var env_map = try std.process.getEnvMap(allocator);
    defer env_map.deinit();

    const appdata = env_map.get("AppData") orelse ".";

    const game_folder_path = try std.fmt.allocPrint(allocator, "{s}/Flux", .{appdata});
    defer allocator.free(game_folder_path);

    Global.GameFolder = try getOrCreateAbs(game_folder_path, "Failed to create game folder. Should never happen.");

    const gamedata = try std.fs.cwd().openFile("gamedata.fpck1", .{});
    try FPCK1.unpack(gamedata, Global.GameFolder.?, allocator);

    Global.MapsFolder = try getOrCreateChild(Global.GameFolder.?, "maps", "Failed to create maps folder. Should never happen.");
    Global.SkinsFolder = try getOrCreateChild(Global.GameFolder.?, "skins", "Failed to create skins folder. Should never happen?");

    var window = try Window.init(allocator);
    window.run() catch |err| {
        std.log.err("Failed to run step game: {any}", .{err});
    };
    defer window.deinit();
}
