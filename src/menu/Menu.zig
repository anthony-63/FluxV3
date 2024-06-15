const std = @import("std");

const Root = @import("../ui/Root.zig");

Root: Root,

pub fn init(allocator: std.mem.Allocator) !@This() {
    return .{
        .Root = Root.init(allocator),
    };
}

pub fn draw(self: @This()) void {
    self.Root.draw();
}
