const std = @import("std");
const rl = @import("raylib");

const Label = @import("Label.zig");

const UIElement = @import("Element.zig").UIElement;

Size: rl.Vector2,
Children: std.ArrayList(UIElement),
DefaultFont: rl.Font,

pub fn init(allocator: std.mem.Allocator) @This() {
    return .{
        .Size = .{ .x = @floatFromInt(rl.getScreenWidth()), .y = @floatFromInt(rl.getScreenHeight()) },
        .Children = std.ArrayList(UIElement).init(allocator),
        .DefaultFont = rl.getFontDefault(),
    };
}

pub fn addChild(self: *@This(), element: *UIElement) void {
    switch (element.*) {
        .label => element.label.Root = self,
        .root => {
            std.log.err("Cannot have root as child of root(draw) {any}", .{self.Children.items});
            std.process.exit(1);
        },
    }

    self.Children.append(element.*) catch {};
}

pub fn draw(self: @This()) void {
    for (self.Children.items) |elem| {
        switch (elem) {
            .label => |*label| label.draw(self.DefaultFont) catch {},
            .root => {
                std.log.err("Cannot have root as child of root(draw) {any}", .{self.Children.items});
                std.process.exit(1);
            },
        }
    }
}

pub fn deinit(self: @This()) void {
    self.Children.deinit();
}
