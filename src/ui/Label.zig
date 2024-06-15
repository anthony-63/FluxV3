const std = @import("std");
const rl = @import("raylib");

const Root = @import("Root.zig");

Position: rl.Vector2,
Anchor: rl.Vector2,

Size: rl.Vector2,

Text: []u8,

Root: *Root,

pub fn draw(self: @This()) void {
    _ = self;
}
