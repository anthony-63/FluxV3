const std = @import("std");
const rl = @import("raylib");

const Root = @import("Root.zig");

Position: rl.Vector2,
Anchor: rl.Vector2,

Text: []const u8,
Centered: bool,

Root: *Root,

Allocator: std.mem.Allocator,

pub fn init(text: []const u8, position: rl.Vector2, anchor: rl.Vector2, centered: bool, allocator: std.mem.Allocator) @This() {
    return .{
        .Position = position,
        .Anchor = anchor,
        .Centered = centered,
        .Allocator = allocator,
        .Root = undefined,
        .Text = text,
    };
}

pub fn draw(self: @This(), font: rl.Font) !void {
    const text = try self.Allocator.dupeZ(u8, self.Text);
    defer self.Allocator.free(text);

    const global_pos = self.Root.Size.multiply(self.Anchor).add(self.Position);

    rl.drawTextPro(font, text, global_pos, rl.Vector2.zero(), 0.0, 16, 4, rl.Color.white);
}
