const std = @import("std");

// format spec

// header: first section len(u64) second section len(u64)

// first section: store tree
// format: id(u32) path_len(u32) path

// second section: id(u32) + data
// id(u32) length(u64) data...

// compressed with flate/deflate

fn createFpck1Tree(dir: std.fs.Dir, allocator: std.mem.Allocator) !std.ArrayList(u8) {
    var data = std.ArrayList(u8).init(allocator);

    var id: u32 = 0;

    var walker = try dir.walk(allocator);
    defer walker.deinit();

    std.debug.print("[FPCK1] creating tree\n", .{});
    while (try walker.next()) |entry| {
        if (entry.kind != .file) {
            continue;
        }

        std.debug.print("[FPCK1 STG1] ({any}) file: {any} {any} {s}\n", .{ data.items.len, id, entry.path.len, entry.path });

        var id_buffer = [_]u8{0} ** 4;
        std.mem.writeInt(u32, &id_buffer, id, .little);
        try data.appendSlice(&id_buffer);

        var path_size_buffer = [_]u8{0} ** 4;
        std.mem.writeInt(u32, &path_size_buffer, @intCast(entry.path.len), .little);
        try data.appendSlice(&path_size_buffer);

        try data.appendSlice(entry.path);

        id += 1;
    }

    return data;
}

fn createFpck1Data(dir: std.fs.Dir, allocator: std.mem.Allocator) !std.ArrayList(u8) {
    var data = std.ArrayList(u8).init(allocator);

    var id: u32 = 0;

    var walker = try dir.walk(allocator);
    defer walker.deinit();

    std.debug.print("[FPCK2] creating data section\n", .{});

    while (try walker.next()) |entry| {
        if (entry.kind != .file) {
            continue;
        }

        var file = try dir.openFile(entry.path, .{});
        const file_stat = try file.stat();

        std.debug.print("[FPCK1 STG2] ({any}) file: {any} {any} \n", .{ data.items.len, id, file_stat.size });

        var id_buffer = [_]u8{0} ** 4;
        std.mem.writeInt(u32, &id_buffer, id, .little);
        try data.appendSlice(&id_buffer);

        const buffer = try allocator.alloc(u8, file_stat.size);
        defer allocator.free(buffer);

        _ = try file.readAll(buffer);

        var data_len_buffer = [_]u8{0} ** 8;
        std.mem.writeInt(u64, &data_len_buffer, buffer.len, .little);
        try data.appendSlice(&data_len_buffer);

        try data.appendSlice(buffer);

        id += 1;
    }

    return data;
}

pub fn pack(to_pack: std.fs.Dir, output: std.fs.File, allocator: std.mem.Allocator) !void {
    const tree = createFpck1Tree(to_pack, allocator) catch {
        return error.FailedToCreateTree;
    };
    defer tree.deinit();

    const data = createFpck1Data(to_pack, allocator) catch {
        return error.FailedToCreateData;
    };
    defer data.deinit();

    var full_file = std.ArrayList(u8).init(allocator);
    defer full_file.deinit();

    var section_size_buffer = [_]u8{0} ** 8;

    std.mem.writeInt(u64, &section_size_buffer, tree.items.len, .little);
    try full_file.appendSlice(&section_size_buffer);

    std.mem.writeInt(u64, &section_size_buffer, data.items.len, .little);
    try full_file.appendSlice(&section_size_buffer);

    try full_file.appendSlice(tree.items);
    try full_file.appendSlice(data.items);

    std.debug.print("total archive size before compression: {d}kb\n", .{@as(f64, @floatFromInt(full_file.items.len)) / 1000.0});

    var stream = std.io.fixedBufferStream(full_file.items);

    std.compress.flate.compress(stream.reader(), output.writer(), .{ .level = .level_9 }) catch {
        return error.FailedToCompressFlate;
    };
}

pub fn unpack(file: std.fs.File, output: std.fs.Dir, allocator: std.mem.Allocator) !void {
    var tmp = try std.fs.cwd().createFile("~fpck1.unpacker", .{});
    defer std.fs.cwd().deleteFile("~fpck1.unpacker") catch {};

    try std.compress.flate.decompress(file.reader(), tmp.writer());
    tmp.close();

    tmp = try std.fs.cwd().openFile("~fpck1.unpacker", .{});
    const tmp_stat = try tmp.stat();

    const data = try tmp.readToEndAlloc(allocator, tmp_stat.size);
    defer allocator.free(data);

    tmp.close();

    var data_stream = std.io.fixedBufferStream(data);

    std.debug.print("[FPCK1] getting header file tree\n", .{});

    var tree_size_buffer = [_]u8{0} ** 8;
    _ = try data_stream.read(&tree_size_buffer);
    const tree_size = std.mem.readInt(u64, &tree_size_buffer, .little);

    var file_data_size_buffer = [_]u8{0} ** 8;
    _ = try data_stream.read(&file_data_size_buffer);
    const file_data_size = std.mem.readInt(u64, &file_data_size_buffer, .little);

    const tree = try allocator.alloc(u8, tree_size);
    defer allocator.free(tree);
    _ = try data_stream.read(tree);

    const files = try allocator.alloc(u8, file_data_size);
    defer allocator.free(files);
    _ = try data_stream.read(files);

    std.debug.print("[FPCK1] tree size: {any}\n", .{tree.len});
    std.debug.print("[FPCK1] data size: {any}\n", .{files.len});

    std.debug.print("[FPCK1] decompressing file tree\n", .{});

    var hashmap = std.AutoHashMap(u32, []u8).init(allocator);
    defer hashmap.deinit();

    var tree_stream = std.io.fixedBufferStream(tree);

    while (tree_stream.pos < tree_stream.buffer.len) {
        var id_buffer = [4]u8{ 0, 0, 0, 0 };
        _ = try tree_stream.read(&id_buffer);
        const id = std.mem.readInt(u32, &id_buffer, .little);

        var path_len_buffer = [4]u8{ 0, 0, 0, 0 };
        _ = try tree_stream.read(&path_len_buffer);
        const path_len = std.mem.readInt(u32, &path_len_buffer, .little);

        const path = try allocator.alloc(u8, path_len);
        _ = try tree_stream.read(path);

        try hashmap.put(id, path);

        std.debug.print("[FPCK1 STG1] {any} {any} {s}\n", .{ id, path_len, path });
    }

    std.debug.print("[FPCK1] decompressing file data\n", .{});
    var file_stream = std.io.fixedBufferStream(files);
    while (file_stream.pos < file_stream.buffer.len) {
        var id_buffer = [4]u8{ 0, 0, 0, 0 };
        _ = try file_stream.read(&id_buffer);
        const id = std.mem.readInt(u32, &id_buffer, .little);

        var data_len_buffer = [8]u8{ 0, 0, 0, 0, 0, 0, 0, 0 };
        _ = try file_stream.read(&data_len_buffer);
        const data_len = std.mem.readInt(u64, &data_len_buffer, .little);

        const file_data = try allocator.alloc(u8, data_len);
        defer allocator.free(file_data);
        _ = try file_stream.read(file_data);

        std.debug.print("[FPCK1 STG2] {any} {any} {s}\n", .{ id, data_len, hashmap.get(id).? });

        if (std.mem.containsAtLeast(u8, hashmap.get(id).?, 1, "/") or std.mem.containsAtLeast(u8, hashmap.get(id).?, 1, "\\")) {
            var fixed_path = hashmap.get(id).?;
            for (fixed_path, 0..) |c, i| {
                if (c == '\\') fixed_path[i] = '/';
            }

            const idx = std.mem.lastIndexOf(u8, fixed_path, "/").?;
            var path = try output.makeOpenPath(fixed_path[0 .. idx + 1], .{});
            defer path.close();

            var output_file = try path.createFile(fixed_path[idx + 1 ..], .{});
            defer output_file.close();
            try output_file.writeAll(file_data);
        } else {
            var output_file = try output.createFile(hashmap.get(id).?, .{});
            defer output_file.close();

            try output_file.writeAll(file_data);
        }

        allocator.free(hashmap.get(id).?);
    }
}
