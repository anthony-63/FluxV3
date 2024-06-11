const std = @import("std");

const NoteData = struct { _x: f32, _y: f32, _time: f32 };

Broken: bool,
Version: u8,
Path: []const u8,
Name: []const u8,
Notes: []NoteData,
Id: []const u8,

pub fn loadFromFile(path: []const u8, allocator: std.mem.Allocator) !@This() {
    const diff_file = try std.fs.cwd().openFile(path, .{});
    defer diff_file.close();

    const diff_stat = try diff_file.stat();

    const diff_buf = try diff_file.readToEndAlloc(allocator, diff_stat.size);
    defer allocator.free(diff_buf);

    const DiffFormat = struct {
        _version: u8,
        _name: []const u8,
        _notes: []NoteData,
    };

    const diff = try std.json.parseFromSlice(DiffFormat, allocator, diff_buf, .{});

    return .{
        .Broken = false,
        .Version = diff.value._version,
        .Path = path,
        .Name = diff.value._name,
        .Notes = diff.value._notes,
        .Id = "",
    };
}