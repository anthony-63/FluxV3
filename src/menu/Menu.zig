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
        "Hello from zig! HELLO!",
        rl.Vector2.zero(),
        rl.Vector2.init(0.5, 0.5),
        true,
        16.0,
        allocator,
    ) };

    root.addChild(&test_label);

    return .{
        .Root = root,
        .Test = test_label,
    };
}

pub fn draw(self: *@This()) void {
    self.Root.draw();
}

pub fn update(self: *@This()) void {
    self.Root.update();
}

pub fn deinit(self: @This()) void {
    self.Root.deinit();
}
