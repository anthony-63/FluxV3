const s2s = @import("ext/s2s.zig");

pub var Note: NoteSettings = .{
    .ApproachRate = 36.8,
    .ApproachDistance = 14.0,
    .ApproachTime = 14.0 / 36.8,
    .Pushback = false,
};

pub var Cursor: CursorSettings = .{
    .Scale = 1.0,
    .Sensitivity = 1.5,
};

const NoteSettings = struct {
    ApproachRate: f64,
    ApproachDistance: f64,
    ApproachTime: f64,
    Pushback: bool,
};

const CursorSettings = struct {
    Scale: f64,
    Sensitivity: f64,
};