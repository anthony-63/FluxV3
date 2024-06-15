const Label = @import("Label.zig");
const Root = @import("Root.zig");

pub const UIElement = union(enum) {
    label: Label,
    root: Root,
};
