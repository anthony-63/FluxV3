const std = @import("std");
const rl = @import("raylib");

const Label = @import("Label.zig");

const UIElement = @import("Element.zig").UIElement;

Size: rl.Vector2,
Children: std.ArrayList(UIElement),

pub fn init(allocator: std.mem.Allocator) @This() {
    return .{
        .Size = .{ .x = @floatFromInt(rl.getScreenWidth()), .y = @floatFromInt(rl.getScreenHeight()) },
        .Children = std.ArrayList(UIElement).init(allocator),
    };
}

pub fn draw(self: @This()) void {
    for (self.Children.items) |elem| {
        switch (elem) {
            .label => |*label| label.draw(),
            .root => std.log.err("Cannot have root as child of root", .{}),
        }
    }
}
