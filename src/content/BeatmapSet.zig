const std = @import("std");
const Beatmap = @import("Beatmap.zig");

Broken: bool,
Version: u8,
Hash: []const u8,
Path: []const u8,

Artist: []const u8,
Title: []const u8,

Difficulties: []Beatmap,
Mappers: [][]const u8,
MusicPath: []const u8,
Cover: []const u8,

pub fn loadFromFolder(folder_path: []const u8, allocator: std.mem.Allocator) !void {
    const meta_path = try std.fmt.allocPrint(allocator, "{s}/meta.json", .{folder_path});
    defer allocator.free(meta_path);

    const meta_file = try std.fs.openFileAbsolute(meta_path, .{});
    defer meta_file.close();

    const meta_stat = try meta_file.stat();

    const meta_buf = try meta_file.readToEndAlloc(allocator, meta_stat.size);
    defer allocator.free(meta_buf);

    const MetaFormat = struct {
        _version: u8,
        _title: []const u8,
        _mappers: [][]const u8,
        _music: []const u8,
        _difficulties: [][]const u8,
    };

    const meta = try std.json.parseFromSlice(MetaFormat, allocator, meta_buf, .{});
    defer meta.deinit();

    std.debug.print("{s}", .{meta.value._title});
}
