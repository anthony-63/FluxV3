const rl = @import("raylib");

pub const HitWindow = 0.055;
pub const AABB = (1.75 + 0.525) / 2.0;

X: f64,
Y: f64,
T: f64,

Hit: bool,
Index: usize,
Color: rl.Color,

pub fn init(x: f64, y: f64, time: f64, index: usize, color: rl.Color) @This() {
    return .{
        .X = x,
        .Y = y,
        .T = time,
        .Index = index,
        .Color = color,
        .Hit = false,
    };
}

pub fn inHitWindow(self: @This(), current_time: f64, speed: f64) bool {
    return (current_time - self.T) <= HitWindow * speed;
}

pub fn isVisisble(self: @This(), current_time: f64, speed: f64, approach_time: f64, pushback: bool) bool {
    if (self.Hit) return false;
    if (current_time > self.T and !pushback) return false;
    return self.calculateTime(current_time, approach_time) <= 1 and self.inHitWindow(current_time, speed);
}

pub fn calculateTime(self: @This(), current_time: f64, approach_time: f64) f64 {
    return (self.T - current_time) / approach_time;
}

pub fn isBeingHit(self: @This(), cursor_pos: rl.Vector2) bool {
    return @abs(cursor_pos.x - self.X * 2) <= AABB and @abs(cursor_pos.y - self.Y * 2) <= AABB;
}
