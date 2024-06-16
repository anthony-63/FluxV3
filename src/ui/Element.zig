const Label = @import("Label.zig");
const Root = @import("Root.zig");

const UIElementTag = enum {
    label,
    root,
};

pub const UIElement = union(UIElementTag) {
    label: Label,
    root: Root,
};
