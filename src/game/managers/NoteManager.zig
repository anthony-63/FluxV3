const std = @import("std");
const rl = @import("raylib");

const Settings = @import("../../Settings.zig");
const Global = @import("../../Global.zig");

const Note = @import("../objects/Note.zig");
const SyncManager = @import("SyncManager.zig");
const NoteRenderer = @import("NoteRenderer.zig");

OrderedNotes: []Note,

NextNote: ?Note,
LastNote: ?Note,

ApproachTime: f64,
SkippedNotes: usize,

StartProcess: usize,

SyncManager: *SyncManager,
NoteRenderer: *NoteRenderer,

Colors: [2]rl.Color,
ToRender: usize,

Pushback: bool,
Started: bool,

pub fn init(sync_manager: *SyncManager, note_renderer: *NoteRenderer, allocator: std.mem.Allocator) !@This() {
    var self: @This() = .{
        .OrderedNotes = undefined,
        .NextNote = undefined,
        .LastNote = undefined,
        .ApproachTime = Settings.Note.ApproachTime,
        .SkippedNotes = 0,
        .StartProcess = 0,
        .ToRender = 0,

        .SyncManager = sync_manager,
        .NoteRenderer = note_renderer,

        .Colors = [2]rl.Color{
            rl.Color.pink,
            rl.Color.white,
        },

        .Pushback = Settings.Note.Pushback,
        .Started = false,
    };

    try self.loadNotes(allocator);

    return self;
}

pub fn update(self: *@This()) !void {
    try self.updateRender();
}

fn updateRender(self: *@This()) !void {
    self.NoteRenderer.ToRender.clearRetainingCapacity();

    self.ToRender = 0;

    for (self.StartProcess..self.OrderedNotes.len) |i| {
        const note = self.OrderedNotes[i];
        if (note.isVisisble(self.SyncManager.RealTime, self.SyncManager.Speed, self.ApproachTime, self.Pushback)) {
            try self.NoteRenderer.ToRender.append(note);
            self.ToRender += 1;
        }

        if (note.T > self.SyncManager.RealTime + self.ApproachTime * self.SyncManager.Speed) break;
    }
    std.debug.print("torender: {d}\n", .{self.ToRender});
}

fn loadNotes(self: *@This(), allocator: std.mem.Allocator) !void {
    self.OrderedNotes = try allocator.alloc(Note, Global.SelectedBeatmapSet.?.Difficulties[Global.SelectedDiffIndex].Notes.len);

    for (Global.SelectedBeatmapSet.?.Difficulties[Global.SelectedDiffIndex].Notes, 0..) |ndata, i| {
        if (ndata._time < Global.StartFrom) {
            self.SkippedNotes += 1;
            continue;
        }

        self.OrderedNotes[i] = .{
            .Color = self.Colors[i % self.Colors.len],
            .Hit = false,
            .Index = i,
            .T = ndata._time,
            .X = ndata._x,
            .Y = ndata._y,
        };
    }

    if (self.OrderedNotes.len > 0) {
        self.NextNote = self.OrderedNotes[0];
    }

    std.debug.print("loaded {any} notes\n", .{self.OrderedNotes.len});
}

pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
    allocator.free(self.OrderedNotes);
}
