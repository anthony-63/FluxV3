const rl = @import("raylib");

Position: rl.Vector3,
Rotation: rl.Vector3,
Size: rl.Vector2,

Model: rl.Model,

Loaded: bool,

pub fn init(texture_path: [:0]const u8) !@This() {
    const size = rl.Vector2.init(6, 6);
    const position = rl.Vector3.init(0, 0, 0);
    const rotation = rl.Vector3.init(90, 0, 0);

    const model = rl.loadModelFromMesh(rl.genMeshPlane(size.x, size.y, 1, 1));
    const img = rl.loadImage(texture_path);

    const tex = rl.loadTextureFromImage(img);
    img.unload();

    rl.setMaterialTexture(model.materials, rl.MATERIAL_MAP_DIFFUSE, tex);

    return .{
        .Position = position,
        .Rotation = rotation,
        .Size = size,

        .Model = model,
        .Loaded = true,
    };
}

pub fn draw(self: @This()) void {
    rl.gl.rlPushMatrix();
    rl.gl.rlRotatef(self.Rotation.x, 1, 0, 0);
    rl.gl.rlRotatef(self.Rotation.y, 0, 1, 0);
    rl.gl.rlRotatef(self.Rotation.z, 0, 0, 1);
    rl.drawModel(self.Model, self.Position, 1, rl.Color.white);
    rl.gl.rlPopMatrix();
}
