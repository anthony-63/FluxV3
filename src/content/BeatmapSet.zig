const std = @import("std");

const Global = @import("../Global.zig");
const Beatmap = @import("Beatmap.zig");

const MetaFormat = struct {
    _version: u8,
    _title: []const u8,
    _mappers: [][]const u8,
    _music: []const u8,
    _difficulties: [][]const u8,
};

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

Parser: std.json.Parsed(MetaFormat),

pub fn loadFromFolder(folder_path: []const u8, allocator: std.mem.Allocator) !@This() {
    const map_folder = Global.MapsFolder.?.openDir(folder_path, .{}) catch {
        return error.FailedToOpenMapFolder;
    };

    const meta_file = map_folder.openFile("meta.json", .{}) catch {
        return error.FailedToOpenMeta;
    };
    defer meta_file.close();

    const meta_stat = try meta_file.stat();

    const meta_buf = try meta_file.readToEndAlloc(allocator, meta_stat.size);
    defer allocator.free(meta_buf);

    const meta = try std.json.parseFromSlice(MetaFormat, allocator, meta_buf, .{ .allocate = .alloc_always });

    var diffs = try allocator.alloc(Beatmap, meta.value._difficulties.len);

    for (meta.value._difficulties, 0..) |diff_file_path, i| {
        const diff_file = map_folder.openFile(diff_file_path, .{}) catch {
            return error.FailedToOpenDiff;
        };
        diffs[i] = Beatmap.loadFromFile(diff_file, allocator) catch {
            return error.FailedToParseDiff;
        };
        diff_file.close();
    }

    const music_file_path = map_folder.realpathAlloc(allocator, meta.value._music) catch {
        return error.FailedToFindMusic;
    };

    return .{
        .Broken = false,
        .Version = meta.value._version,
        .Path = folder_path,
        .Artist = "",
        .Hash = "",
        .Title = meta.value._title,
        .Difficulties = diffs,
        .Mappers = meta.value._mappers,
        .MusicPath = music_file_path,
        .Parser = meta,
    };
}

pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
    self.Parser.deinit();
    for (self.Difficulties) |diff| {
        diff.deinit();
    }
    allocator.free(self.Difficulties);
    allocator.free(self.MusicPath);
}
