const std = @import("std");
const rl = @import("raylib");

Music: rl.Music,
Stream: []u8,

pub fn init(path: []const u8, allocator: std.mem.Allocator) !@This() {
    const audio_file = try std.fs.cwd().openFile(path, .{});
    defer audio_file.close();

    const audio_stat = try audio_file.stat();

    const audio_buf = try audio_file.readToEndAlloc(allocator, audio_stat.size);

    return .{
        .Music = rl.loadMusicStreamFromMemory(".mp3", audio_buf),
        .Stream = audio_buf,
    };
}

pub fn play(self: @This(), from: f64) void {
    rl.seekMusicStream(self.Music, @floatCast(from));
    rl.playMusicStream(self.Music);
}

pub fn update(self: @This()) void {
    rl.updateMusicStream(self.Music);
}

pub fn shouldSync(self: @This(), current_time: f64) bool {
    return @abs(current_time - rl.getMusicTimePlayed(self.Music)) >= 0.05;
}

pub fn sync(self: @This(), current_time: f64) void {
    if (self.shouldSync(current_time)) {
        std.log.info("synced audio to: {any}", .{current_time});
        rl.seekMusicStream(self.Music, @floatCast(current_time));
    }
}

pub fn setVolume(self: @This(), volume: f64) void {
    rl.setMusicVolume(self.Music, @floatCast(volume));
}

pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
    allocator.free(self.Stream);
}
