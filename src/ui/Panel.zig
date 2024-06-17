const std = @import("std");
const rl = @import("raylib");

const Root = @import("Root.zig");

Position: rl.Vector2,
Size: rl.Vector2,
Anchor: rl.Vector2,

BorderColor: rl.Color,
BorderSize: f32,
Rounded: f32,

Centered: bool,

Color: rl.Color,

Root: ?Root,

pub fn init(position: rl.Vector2, size: rl.Vector2, anchor: rl.Vector2, color: rl.Color, border_color: rl.Color, border_size: f32, rounded: f32, centered: bool) @This() {
    return .{
        .Position = position,
        .Size = size,
        .Anchor = anchor,
        .Centered = centered,
        .Color = color,
        .Root = undefined,
        .BorderSize = border_size,
        .BorderColor = border_color,
        .Rounded = rounded,
    };
}

pub fn draw(self: @This()) void {
    var global_pos = self.Root.?.getGlobalPosition(self.Position, self.Anchor);

    if (self.Centered) {
        global_pos = global_pos.subtract(self.Size.multiply(rl.Vector2.init(0.5, 0.5)));
    }

    if (self.BorderSize > 0) {
        const border_rect = rl.Rectangle.init(global_pos.x - (self.BorderSize / 2), global_pos.y - (self.BorderSize / 2), self.Size.x + self.BorderSize, self.Size.y + self.BorderSize);
        if (self.Rounded > 0) {
            rl.drawRectangleRounded(border_rect, self.Rounded, 100, self.BorderColor);
        } else rl.drawRectanglePro(border_rect, rl.Vector2.zero(), 0, self.BorderColor);
    }
    const rect = rl.Rectangle.init(global_pos.x, global_pos.y, self.Size.x, self.Size.y);

    if (self.Rounded > 0) {
        rl.drawRectangleRounded(rect, self.Rounded, 100, self.Color);
    } else rl.drawRectanglePro(rect, rl.Vector2.zero(), 0, self.Color);
}
