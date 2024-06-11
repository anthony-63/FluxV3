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
Cover: ?[]const u8 = null,

pub fn loadFromFolder(folder_path: []const u8, allocator: std.mem.Allocator) !@This() {
    const meta_path = try std.fmt.allocPrint(allocator, "{s}/meta.json", .{folder_path});
    defer allocator.free(meta_path);

    const meta_file = try std.fs.cwd().openFile(meta_path, .{});
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

    var diffs = try allocator.alloc(Beatmap, meta.value._difficulties.len);
    defer allocator.free(diffs);

    for (meta.value._difficulties, 0..) |diff_file_path, i| {
        const diff_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ folder_path, diff_file_path });
        diffs[i] = try Beatmap.loadFromFile(diff_path, allocator);
        allocator.free(diff_path);
    }

    return .{
        .Broken = false,
        .Version = meta.value._version,
        .Path = folder_path,
        .Artist = "",
        .Hash = "",
        .Title = meta.value._title,
        .Difficulties = diffs,
        .Mappers = meta.value._mappers,
        .MusicPath = meta.value._music,
    };
}
