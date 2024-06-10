const NoteData = struct { x: f32, y: f32, t: f32 };

Broken: bool,
Version: u8,
Path: []const u8,
Name: []const u8,
Notes: []NoteData,
Id: []const u8,
