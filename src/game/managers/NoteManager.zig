const std = @import("std");
const rl = @import("raylib");

const Settings = @import("../../Settings.zig");
const Global = @import("../../Global.zig");

const Note = @import("../objects/Note.zig");
const SyncManager = @import("SyncManager.zig");
const NoteRenderer = @import("NoteRenderer.zig");

OrderedNotes: []Note,

NextNote: *Note,
LastNote: *Note,

ApproachTime: f64,
SkippedNotes: usize,

StartProcess: usize,

Colors: [2]rl.Color,

ToUpdateIndices: std.ArrayList(usize),

Pushback: bool,
Started: bool,

pub fn init(allocator: std.mem.Allocator) !@This() {
    var self: @This() = .{
        .OrderedNotes = undefined,
        .NextNote = undefined,
        .LastNote = undefined,
        .ApproachTime = Settings.Note.ApproachTime,
        .SkippedNotes = 0,
        .StartProcess = 0,

        .ToUpdateIndices = std.ArrayList(usize).init(allocator),

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

pub fn update(self: *@This(), renderer: *NoteRenderer, sync: SyncManager) !void {
    if (self.NextNote == undefined) return;
    try self.updateNotes(sync);
    try self.updateRender(renderer, sync);
}

fn updateRender(self: *@This(), renderer: *NoteRenderer, sync: SyncManager) !void {
    renderer.ToRender.clearRetainingCapacity();

    for (self.StartProcess..self.OrderedNotes.len) |i| {
        const note = self.OrderedNotes[i];
        if (note.isVisisble(sync.RealTime, sync.Speed, self.ApproachTime, self.Pushback)) {
            try renderer.ToRender.append(note);
        }

        if (note.T > sync.RealTime + self.ApproachTime * sync.Speed) break;
    }
}

fn updateNotes(self: *@This(), sync: SyncManager) !void {
    // std.debug.print("{d}\n", .{self.StartProcess});
    self.ToUpdateIndices.clearRetainingCapacity();

    for (self.StartProcess..self.OrderedNotes.len) |i| {
        var note = self.OrderedNotes[i];
        if (note.calculateTime(sync.RealTime, self.ApproachTime * sync.Speed) <= 0 and !note.Hit) {
            try self.ToUpdateIndices.append(i);
        }

        if (note.T > sync.RealTime + self.ApproachTime * sync.Speed) {
            break;
        }
    }

    for (self.ToUpdateIndices.items) |i| {
        var did_hitreg = false;

        if (false) { // check if note is being hit
            self.OrderedNotes[i].Hit = true;
            did_hitreg = true;
        }

        if (!self.OrderedNotes[i].Hit and !self.OrderedNotes[i].inHitWindow(sync.RealTime, sync.Speed)) {
            self.OrderedNotes[i].Hit = true;
            did_hitreg = true;
        }

        if (did_hitreg) {
            self.LastNote = &self.OrderedNotes[i];
            if (self.OrderedNotes[i].Index < self.OrderedNotes.len - 1) {
                self.NextNote = &self.OrderedNotes[self.OrderedNotes[i].Index + 1];
                self.StartProcess += 1;
            } else {
                self.NextNote = undefined;
            }
        }
    }
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
        self.NextNote = &self.OrderedNotes[0];
    }

    std.debug.print("loaded {any} notes\n", .{self.OrderedNotes.len});
}

pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
    allocator.free(self.OrderedNotes);
    self.ToUpdateIndices.deinit();
}
