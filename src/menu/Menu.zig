const std = @import("std");
const rl = @import("raylib");

const Root = @import("../ui/Root.zig");
const Label = @import("../ui/Label.zig");
const Panel = @import("../ui/Panel.zig");
const UIElement = @import("../ui/Element.zig").UIElement;

Root: Root,
Test: UIElement,

pub fn init(allocator: std.mem.Allocator) !@This() {
    var root = Root.init(allocator);
    var test_panel = UIElement{ .panel = Panel.init(
        rl.Vector2.init(0, 0),
        rl.Vector2.init(50, 50),
        rl.Vector2.init(0.5, 0.5),
        rl.Color.white,
        true,
    ) };

    root.addChild(&test_panel);

    return .{
        .Root = root,
        .Test = test_panel,
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
