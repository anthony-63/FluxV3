const rl = @import("raylib");

pub fn init() !@This() {
    return .{};
}

pub fn draw(self: @This()) void {
    _ = self;
    rl.clearBackground(rl.Color.blue);
}

pub fn update(self: @This()) void {
    _ = self;
}
