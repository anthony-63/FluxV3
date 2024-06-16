const std = @import("std");
const rl = @import("raylib");

const Game = @import("../game/Game.zig");
const Menu = @import("../menu/Menu.zig");

const Global = @import("../Global.zig");
const BeatmapSet = @import("../content/BeatmapSet.zig");

pub const WindowState = enum {
    LOADING,
    MENU,
    GAME,
};

CurrentState: WindowState,
Allocator: std.mem.Allocator,

Game: ?Game,
Menu: ?Menu,

pub fn init(allocator: std.mem.Allocator) !@This() {
    rl.setConfigFlags(.{ .window_resizable = true });

    rl.initWindow(1280, 720, "FluxV3-OPT");
    rl.initAudioDevice();
    return .{
        .Allocator = allocator,
        .CurrentState = .LOADING,
        .Game = null,
        .Menu = null,
    };
}

pub fn run(self: *@This()) !void {
    while (!rl.windowShouldClose()) {
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(rl.Color.black);

        switch (self.CurrentState) {
            .LOADING => {
                Global.SelectedBeatmapSet = try BeatmapSet.loadFromFolder("dive_camellia_-_superluminal", self.Allocator);
                self.CurrentState = .MENU;
            },
            .MENU => {
                if (self.Menu == null) {
                    self.Menu = try Menu.init(self.Allocator);
                } else {
                    self.Menu.?.update();
                    self.Menu.?.draw();
                }
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
    if (self.Menu != null) {
        self.Menu.?.deinit();
    }
    Global.deinit(self.Allocator);
    rl.closeWindow();
}
