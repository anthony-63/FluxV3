const std = @import("std");
const rl = @import("raylib");

const Root = @import("Root.zig");

Position: rl.Vector2,
Anchor: rl.Vector2,

Text: []const u8,
FontSize: f32,
Centered: bool,

Root: ?Root,

Allocator: std.mem.Allocator,

pub fn init(text: []const u8, position: rl.Vector2, anchor: rl.Vector2, centered: bool, font_size: f32, allocator: std.mem.Allocator) @This() {
    return .{
        .Position = position,
        .Anchor = anchor,
        .Centered = centered,
        .Allocator = allocator,
        .Root = undefined,
        .Text = text,
        .FontSize = font_size,
    };
}

pub fn draw(self: @This(), font: rl.Font) !void {
    const text = try self.Allocator.dupeZ(u8, self.Text);
    defer self.Allocator.free(text);

    var global_pos = self.Root.?.Size.multiply(self.Anchor).add(self.Position);

    if (self.Centered) {
        global_pos = global_pos.subtract(rl.measureTextEx(font, text, self.FontSize, 2).multiply(rl.Vector2.init(0.5, 0.5)));
    }

    rl.drawTextPro(font, text, global_pos, rl.Vector2.zero(), 0.0, self.FontSize, 2, rl.Color.white);
}
