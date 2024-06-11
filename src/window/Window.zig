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
Game: Game,
Allocator: std.mem.Allocator,

pub fn init(allocator: std.mem.Allocator) !@This() {
    rl.initWindow(1280, 720, "FluxV3-OPT");

    return .{
        .Allocator = allocator,
        .CurrentState = .LOADING,
        .Game = try Game.init(),
    };
}

pub fn run(self: *@This()) !void {
    while (!rl.windowShouldClose()) {
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(rl.Color.black);

        switch (self.CurrentState) {
            .LOADING => {
                Global.SelectedBeatmapSet = try BeatmapSet.loadFromFolder("C:/Users/antho/AppData/Roaming/Flux/maps/zitronitro_7_7_-_mii_munbe_plaza", self.Allocator);
                self.CurrentState = .GAME;
            },
            .GAME => {
                self.Game.update();
                self.Game.draw();
            },
        }
    }
}

pub fn deinit(self: @This()) void {
    rl.closeWindow();
    Global.deinit(self.Allocator);
}
