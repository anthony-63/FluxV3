const s2s = @import("ext/s2s.zig");

pub var Note: NoteSettings = .{
    .ApproachRate = 36.8,
    .ApproachDistance = 14.0,
    .ApproachTime = 14.0 / 36.8,
    .Pushback = false,
};

pub var Cursor: CursorSettings = .{
    .Scale = 0.8,
    .Sensitivity = 1.5,
    .Drift = true,
};

pub var Camera: CameraSettings = .{
    .Parallax = 10.0,
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
    Drift: bool,
};

const CameraSettings = struct {
    Parallax: f64,
};
