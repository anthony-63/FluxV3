const std = @import("std");
const rl = @import("raylib");

const Root = @import("../ui/Root.zig");
const Label = @import("../ui/Label.zig");
const UIElement = @import("../ui/Element.zig").UIElement;

Root: Root,
Test: UIElement,

pub fn init(allocator: std.mem.Allocator) !@This() {
    var root = Root.init(allocator);
    var test_label = UIElement{ .label = Label.init(
        "Hello",
        .{ .x = 100, .y = 100 },
        rl.Vector2.zero(),
        false,
        allocator,
    ) };

    root.addChild(&test_label);

    return .{
        .Root = root,
        .Test = test_label,
    };
}

pub fn draw(self: @This()) void {
    self.Root.draw();
}

pub fn deinit(self: @This()) void {
    self.Root.deinit();
}
