const std = @import("std");
const rl = @import("raylib");

const Camera = @import("Camera.zig");

const Global = @import("../../Global.zig");
const Settings = @import("../../Settings.zig");

const CLAMP = ((6.0 - 0.525) / 2.0);

Model: rl.Model,
Rotation: rl.Vector3,
Position: rl.Vector3,

RealPosition: rl.Vector2,
ClampedPosition: rl.Vector2,

LastMousePos: rl.Vector2,

pub fn init(texture_path: []const u8, allocator: std.mem.Allocator) !@This() {
    const size = rl.Vector2.init(@floatCast(Settings.Cursor.Scale), @floatCast(Settings.Cursor.Scale));
    const position = rl.Vector3.init(0, 0, 0);
    const rotation = rl.Vector3.init(90, 0, 0);

    const model = rl.loadModelFromMesh(rl.genMeshPlane(size.x, size.y, 1, 1));

    var buf: [std.fs.max_path_bytes]u8 = undefined;
    const full_tex_path = try allocator.dupeZ(u8, try Global.SkinsFolder.?.realpath(texture_path, buf[0..]));
    defer allocator.free(full_tex_path);

    const img = rl.loadImage(full_tex_path);

    const tex = rl.loadTextureFromImage(img);
    img.unload();

    rl.setMaterialTexture(model.materials, rl.MATERIAL_MAP_DIFFUSE, tex);

    return .{
        .Model = model,
        .Position = position,
        .Rotation = rotation,

        .RealPosition = rl.Vector2.init(0, 0),
        .ClampedPosition = rl.Vector2.init(0, 0),

        .LastMousePos = rl.Vector2.init(0, 0),
    };
}

pub fn draw(self: @This()) void {
    rl.gl.rlPushMatrix();
    rl.gl.rlRotatef(self.Rotation.x, 1, 0, 0);
    rl.gl.rlRotatef(self.Rotation.y, 0, 1, 0);
    rl.gl.rlRotatef(self.Rotation.z, 0, 0, 1);
    self.Model.draw(self.Position, 1, rl.Color.white);
    rl.gl.rlPopMatrix();
}

pub fn update(self: *@This(), camera: *Camera) void {
    const mouse_now = rl.getMousePosition();
    const mouse_delta = mouse_now.subtract(self.LastMousePos);

    self.LastMousePos = mouse_now;

    const sens_factor = @as(f32, @floatCast(Settings.Cursor.Sensitivity / 100.0));

    const delta = rl.Vector2.init(
        mouse_delta.x * sens_factor,
        mouse_delta.y * sens_factor,
    );

    self.RealPosition = self.RealPosition.add(delta);

    if (Settings.Cursor.Drift) {
        self.RealPosition = rl.Vector2.init(std.math.clamp(self.RealPosition.x, -CLAMP, CLAMP), std.math.clamp(self.RealPosition.y, -CLAMP, CLAMP));
        self.ClampedPosition = self.RealPosition;
    } else {
        self.ClampedPosition = rl.Vector2.init(std.math.clamp(self.RealPosition.x, -CLAMP, CLAMP), std.math.clamp(self.RealPosition.y, -CLAMP, CLAMP));
    }

    self.Position = rl.Vector3.init(self.ClampedPosition.x, 0, self.ClampedPosition.y);
    camera.doParallax(self.ClampedPosition);
}
