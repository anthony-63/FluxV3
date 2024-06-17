const Label = @import("Label.zig");
const Panel = @import("Panel.zig");
const Root = @import("Root.zig");

const UIElementTag = enum {
    label,
    panel,
    root,
};

pub const UIElement = union(UIElementTag) {
    label: Label,
    panel: Panel,
    root: Root,
};
