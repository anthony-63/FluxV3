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
