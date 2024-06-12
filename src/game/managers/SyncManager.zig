const std = @import("std");

const Global = @import("../../Global.zig");
const MusicPlayer = @import("../objects/MusicPlayer.zig");

Playing: bool,
Speed: f64,

LastTime: f64,
RealTime: f64,
EndTime: f64,

MusicPlayer: MusicPlayer,

pub fn init(allocator: std.mem.Allocator) !@This() {
    return .{
        .Speed = 1,

        .Playing = false,
        .LastTime = 0,
        .RealTime = 0,
        .EndTime = Global.SelectedBeatmapSet.?.Difficulties[Global.SelectedDiffIndex].Notes[Global.SelectedBeatmapSet.?.Difficulties[Global.SelectedDiffIndex].Notes.len - 1]._time,

        .MusicPlayer = try MusicPlayer.init(Global.SelectedBeatmapSet.?.MusicPath, allocator),
    };
}

pub fn update(self: *@This()) void {
    if (!self.Playing) return;

    self.MusicPlayer.update();
    self.MusicPlayer.sync(self.RealTime);

    const now = @as(f64, @floatFromInt(std.time.microTimestamp()));
    const time = self.Speed * (now - self.LastTime) * 0.000001;
    self.LastTime = now;
    self.RealTime += time;
}

pub fn start(self: *@This(), from: f64) void {
    self.LastTime = @floatFromInt(std.time.microTimestamp());
    self.RealTime = from;
    self.Playing = true;
    self.MusicPlayer.play(from);
    self.MusicPlayer.setVolume(0.1);
}

pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
    self.MusicPlayer.deinit(allocator);
}
