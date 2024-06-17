const std = @import("std");
const rl = @import("raylib");

const Root = @import("Root.zig");

Position: rl.Vector2,
Size: rl.Vector2,
Anchor: rl.Vector2,

Centered: bool,

Color: rl.Color,

Root: ?Root,

pub fn init(position: rl.Vector2, size: rl.Vector2, anchor: rl.Vector2, color: rl.Color, centered: bool) @This() {
    return .{
        .Position = position,
        .Size = size,
        .Anchor = anchor,
        .Centered = centered,
        .Color = color,
        .Root = undefined,
    };
}

pub fn draw(self: @This()) void {
    const global_pos = self.Root.?.getGlobalPosition(self.Position, self.Anchor);

    const origin = if (!self.Centered) rl.Vector2.init(0, 0) else self.Size.multiply(rl.Vector2.init(0.5, 0.5));
    const rect = rl.Rectangle.init(global_pos.x, global_pos.y, self.Size.x, self.Size.y);

    rl.drawRectanglePro(rect, origin, 0, self.Color);
}
