const std = @import("std");
const rl = @import("raylib");

const Game = @import("../game/Game.zig");
const Global = @import("../Global.zig");
const BeatmapSet = @import("../content/BeatmapSet.zig");

pub const WindowState = enum {
    LOADING,
    GAME,
};

CurrentState: WindowState,
Game: ?Game,
Allocator: std.mem.Allocator,

pub fn init(allocator: std.mem.Allocator) !@This() {
    rl.initWindow(1280, 720, "FluxV3-OPT");
    rl.initAudioDevice();

    return .{
        .Allocator = allocator,
        .CurrentState = .LOADING,
        .Game = null,
    };
}

pub fn run(self: *@This()) !void {
    while (!rl.windowShouldClose()) {
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(rl.Color.black);

        switch (self.CurrentState) {
            .LOADING => {
                Global.SelectedBeatmapSet = try BeatmapSet.loadFromFolder("wasteful_succducc_me_u", self.Allocator);
                self.CurrentState = .GAME;
            },
            .GAME => {
                if (self.Game == null) {
                    self.Game = try Game.init(self.Allocator);
                } else {
                    try self.Game.?.update();
                    self.Game.?.draw();
                }
            },
        }
    }
}

pub fn deinit(self: @This()) void {
    if (self.Game != null) {
        self.Game.?.deinit();
    }
    Global.deinit(self.Allocator);
    rl.closeWindow();
}
