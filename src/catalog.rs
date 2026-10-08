//! The tool catalogue and command registry — the same "All tools" information architecture
//! PrintCraft ships in `crates/engine/src/catalog.rs`, reduced to the rows this demo renders.
//!
//! Nothing in the UI hard-codes a tool: the left panel, the command palette and the home screen
//! all iterate these tables, exactly like PrintCraft's egui shell.

/// Where a tool or item stands. Planned entries stay visible but disabled, so the UI is honest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Availability {
    Ready,
    Planned(&'static str),
    Provider,
}

impl Availability {
    pub fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// A short pill for the row ("M7", "Provider").
    pub fn badge(self) -> Option<&'static str> {
        match self {
            Self::Ready => None,
            Self::Planned(milestone) => Some(milestone),
            Self::Provider => Some("Provider"),
        }
    }

    pub fn note(self) -> &'static str {
        match self {
            Self::Ready => "Works in this build",
            Self::Planned(milestone) => milestone,
            Self::Provider => "Optional pluggable provider",
        }
    }
}

/// One actionable row inside a tool's detail panel.
#[derive(Clone, Copy, Debug)]
pub struct ToolItem {
    pub label: &'static str,
    pub icon: &'static str,
    pub command: &'static str,
    pub availability: Availability,
}

/// A titled group of rows ("Markup", "Drawing", "Review").
#[derive(Clone, Copy, Debug)]
pub struct ToolSection {
    pub title: &'static str,
    pub items: &'static [ToolItem],
}

/// A tool family, as it appears in "All tools" and in the command palette.
#[derive(Clone, Copy, Debug)]
pub struct ToolGroup {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    /// Icon tint; PrintCraft colour-codes its tool families the way Acrobat does.
    pub hue: [u8; 3],
    pub availability: Availability,
    pub sections: &'static [ToolSection],
}

impl ToolGroup {
    /// The one-line blurb shown under a tool's title in the detail panel.
    pub fn blurb(&self) -> String {
        self.sections
            .first()
            .map(|s| s.items.iter().take(3).map(|i| i.label).collect::<Vec<_>>().join(" · "))
            .unwrap_or_default()
    }

    pub fn item_count(&self) -> usize {
        self.sections.iter().map(|s| s.items.len()).sum()
    }

    pub fn find(id: &str) -> Option<&'static ToolGroup> {
        TOOL_GROUPS.iter().find(|g| g.id == id)
    }
}

const RED: [u8; 3] = [0xE0, 0x3E, 0x3E];
const PURPLE: [u8; 3] = [0x8E, 0x4E, 0xE6];
const BLUE: [u8; 3] = [0x3A, 0x6F, 0xE8];
const GREEN: [u8; 3] = [0x2D, 0x9D, 0x5B];
const ORANGE: [u8; 3] = [0xE8, 0x8A, 0x1A];
const TEAL: [u8; 3] = [0x14, 0x9C, 0xA8];
const PINK: [u8; 3] = [0xD6, 0x3B, 0x8F];

const fn item(label: &'static str, icon: &'static str, command: &'static str, availability: Availability) -> ToolItem {
    ToolItem { label, icon, command, availability }
}

use Availability::{Planned, Provider, Ready};

/// Every tool family the demo knows about, in Acrobat's order.
pub static TOOL_GROUPS: &[ToolGroup] = &[
    ToolGroup {
        id: "organize",
        label: "Organize pages",
        icon: "layout-grid",
        hue: GREEN,
        availability: Ready,
        sections: &[
            ToolSection {
                title: "Page options",
                items: &[
                    item("Page grid", "layout-grid", "page.organize", Ready),
                    item("Rotate", "rotate-cw", "page.rotate", Ready),
                    item("Delete", "trash-2", "page.delete", Ready),
                    item("Extract", "file-output", "page.extract", Ready),
                    item("Insert", "file-plus-2", "page.insert", Ready),
                    item("Replace", "replace", "page.replace", Ready),
                    item("Split", "scissors", "page.split", Ready),
                ],
            },
            ToolSection {
                title: "More",
                items: &[
                    item("Set page boxes", "square-dashed-mouse-pointer", "page.boxes", Ready),
                    item("Crop pages", "crop", "page.crop", Ready),
                    item("Duplicate pages", "copy-plus", "page.duplicate", Ready),
                    item("Page labels", "tag", "page.number", Ready),
                    item("Page transitions", "presentation", "page.transitions", Planned("M4")),
                ],
            },
        ],
    },
    ToolGroup {
        id: "comment",
        label: "Add comments",
        icon: "message-square-text",
        hue: ORANGE,
        availability: Ready,
        sections: &[
            ToolSection {
                title: "Markup",
                items: &[
                    item("Sticky note", "sticky-note", "comment.note", Ready),
                    item("Highlight text", "highlighter", "comment.highlight", Ready),
                    item("Underline", "underline", "comment.underline", Ready),
                    item("Strikethrough", "strikethrough", "comment.strikeout", Ready),
                    item("Text box", "type", "comment.freetext", Ready),
                ],
            },
            ToolSection {
                title: "Drawing",
                items: &[
                    item("Draw freehand", "pencil", "comment.ink", Ready),
                    item("Rectangle", "square", "comment.square", Ready),
                    item("Oval", "circle", "comment.circle", Ready),
                    item("Line", "minus", "comment.line", Ready),
                    item("Arrow", "move-right", "comment.arrow", Ready),
                    item("Cloud", "cloud", "comment.cloud", Planned("M5")),
                    item("Stamp", "stamp", "comment.stamp", Planned("M5")),
                ],
            },
            ToolSection {
                title: "Review",
                items: &[
                    item("Comment list", "message-square-text", "comment.list", Ready),
                    item("Flatten comments", "layers", "comment.flatten", Ready),
                ],
            },
        ],
    },
    ToolGroup {
        id: "fill_sign",
        label: "Fill & Sign",
        icon: "pen-line",
        hue: PURPLE,
        availability: Ready,
        sections: &[ToolSection {
            title: "Fill",
            items: &[
                item("Add text", "type", "sign.fill.text", Ready),
                item("Checkmark", "check", "sign.fill.check", Ready),
                item("Cross", "x", "sign.fill.cross", Ready),
                item("Dot", "circle-dot", "sign.fill.dot", Ready),
                item("Date", "clock-3", "sign.fill.date", Ready),
                item("Signature", "signature", "sign.fill.signature", Ready),
                item("Initials", "signature", "sign.fill.initials", Ready),
            ],
        }],
    },
    ToolGroup {
        id: "edit",
        label: "Edit a PDF",
        icon: "file-pen-line",
        hue: PINK,
        availability: Planned("M7"),
        sections: &[
            ToolSection {
                title: "Modify page",
                items: &[
                    item("Rotate pages", "rotate-cw", "page.rotate", Ready),
                    item("Insert pages", "file-plus-2", "page.insert", Ready),
                    item("Delete pages", "trash-2", "page.delete", Ready),
                    item("Organize pages", "layout-grid", "page.organize", Ready),
                ],
            },
            ToolSection {
                title: "Add content",
                items: &[
                    item("Edit text & images", "text-cursor-input", "edit.edit_text", Ready),
                    item("Text", "type", "edit.text", Ready),
                    item("Image", "image-plus", "edit.image", Ready),
                    item("Header and footer", "heading", "edit.header_footer", Ready),
                    item("Watermark", "stamp", "edit.watermark", Ready),
                    item("Link", "link-2", "edit.link", Ready),
                    item("Bates numbering", "hash", "edit.bates", Ready),
                ],
            },
        ],
    },
    ToolGroup {
        id: "form",
        label: "Prepare a form",
        icon: "text-cursor-input",
        hue: PURPLE,
        availability: Ready,
        sections: &[
            ToolSection {
                title: "Fields",
                items: &[
                    item("Field list", "list", "form.fields", Ready),
                    item("Detect form fields", "scan", "form.detect", Ready),
                    item("Clear form", "eraser", "form.clear", Ready),
                    item("Field properties", "settings-2", "form.field.properties", Ready),
                    item("Flatten form fields", "layers", "form.flatten", Ready),
                    item("Import data", "file-input", "form.import_data", Ready),
                ],
            },
            ToolSection {
                title: "Add form components",
                items: &[
                    item("Text field", "text-cursor-input", "form.add.text", Ready),
                    item("Checkbox", "check-circle-2", "form.add.checkbox", Ready),
                    item("Radio button", "circle", "form.add.radio", Ready),
                    item("Drop-down list", "chevron-down", "form.add.combo", Ready),
                    item("Button", "square", "form.add.button", Ready),
                    item("Date field", "clock-3", "form.add.date", Ready),
                    item("Digital signature", "signature", "form.add.signature", Ready),
                ],
            },
        ],
    },
    ToolGroup {
        id: "export",
        label: "Export a PDF",
        icon: "file-output",
        hue: BLUE,
        availability: Ready,
        sections: &[ToolSection {
            title: "Export to",
            items: &[
                item("Microsoft Word (.docx)", "file-text", "export.docx", Ready),
                item("Spreadsheet (.xlsx)", "grid-3x3", "export.xlsx", Planned("M10")),
                item("Image (PNG)", "image", "export.image", Ready),
                item("Export all images", "image", "export.all_images", Ready),
                item("HTML web page", "file-symlink", "export.html", Ready),
                item("Text (plain)", "type", "export.text", Ready),
            ],
        }],
    },
    ToolGroup {
        id: "create",
        label: "Create a PDF",
        icon: "file-plus-2",
        hue: RED,
        availability: Ready,
        sections: &[ToolSection {
            title: "Create from",
            items: &[
                item("Single file", "file-input", "create.file", Ready),
                item("Multiple files", "files", "create.multiple", Planned("M10")),
                item("Images", "image", "create.images", Ready),
                item("Clipboard", "copy-plus", "create.clipboard", Ready),
                item("Blank page", "file-plus-2", "create.blank", Ready),
                item("Cloud storage (provider)", "cloud", "create.cloud", Provider),
            ],
        }],
    },
    ToolGroup {
        id: "combine",
        label: "Combine files",
        icon: "files",
        hue: BLUE,
        availability: Ready,
        sections: &[ToolSection {
            title: "Combine",
            items: &[item("Add files to combine", "files", "page.combine", Ready)],
        }],
    },
    ToolGroup {
        id: "compress",
        label: "Compress a PDF",
        icon: "file-down",
        hue: RED,
        availability: Ready,
        sections: &[ToolSection {
            title: "Optimize",
            items: &[
                item("Reduce file size", "file-down", "optimize.reduce", Ready),
                item("Advanced optimization", "settings-2", "optimize.advanced", Ready),
                item("Audit space usage", "gauge", "optimize.audit", Planned("M11")),
            ],
        }],
    },
    ToolGroup {
        id: "protect",
        label: "Protect a PDF",
        icon: "shield-check",
        hue: BLUE,
        availability: Planned("M8"),
        sections: &[
            ToolSection {
                title: "Protect",
                items: &[
                    item("Protect with password", "lock", "protect.password", Ready),
                    item("Remove hidden information", "eye-off", "protect.remove_hidden", Ready),
                ],
            },
            ToolSection {
                title: "Advanced options",
                items: &[
                    item("Encrypt with certificate", "file-lock-2", "protect.certificate", Planned("M8")),
                    item("Security properties", "shield-check", "protect.properties", Ready),
                    item("Remove security", "lock-open", "protect.remove", Ready),
                ],
            },
        ],
    },
    ToolGroup {
        id: "redact",
        label: "Redact a PDF",
        icon: "rectangle-horizontal",
        hue: RED,
        availability: Ready,
        sections: &[ToolSection {
            title: "Redact",
            items: &[
                item("Redact text and images", "rectangle-horizontal", "redact.mark", Ready),
                item("Redact pages", "file-x", "redact.pages", Ready),
                item("Find text and redact", "file-search", "redact.search", Ready),
                item("Sanitize document", "sparkles", "redact.sanitize", Ready),
            ],
        }],
    },
    ToolGroup {
        id: "scan",
        label: "Scan & OCR",
        icon: "scan-text",
        hue: GREEN,
        availability: Ready,
        sections: &[ToolSection {
            title: "Recognize text",
            items: &[
                item("In this file", "scan-text", "ocr.recognize", Ready),
                item("In multiple files", "files", "ocr.recognize_batch", Ready),
                item("Enhance scanned file", "sparkles", "ocr.enhance", Planned("M10")),
            ],
        }],
    },
    ToolGroup {
        id: "certificate",
        label: "Use a certificate",
        icon: "badge-check",
        hue: TEAL,
        availability: Ready,
        sections: &[ToolSection {
            title: "Certificates",
            items: &[
                item("Digitally sign", "signature", "sign.digital", Ready),
                item("Timestamp", "clock-3", "sign.timestamp", Planned("M9")),
                item("Validate all signatures", "badge-check", "sign.validate", Ready),
                item("Certify (visible signature)", "badge-check", "sign.certify", Ready),
            ],
        }],
    },
];

/// A registry command: what the palette, the menus and the shortcuts all dispatch by id.
#[derive(Clone, Copy, Debug)]
pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    /// The menu it belongs to.
    pub menu: &'static str,
    pub shortcut: Option<&'static str>,
    pub icon: &'static str,
}

const fn command(
    id: &'static str,
    label: &'static str,
    menu: &'static str,
    shortcut: Option<&'static str>,
    icon: &'static str,
) -> Command {
    Command { id, label, menu, shortcut, icon }
}

pub static COMMANDS: &[Command] = &[
    command("file.open", "Open…", "File", Some("⌘O"), "folder-open"),
    command("file.new", "New blank document", "File", Some("⌘N"), "file-plus-2"),
    command("file.save", "Save", "File", Some("⌘S"), "save"),
    command("file.print", "Print…", "File", Some("⌘P"), "printer"),
    command("file.properties", "Document properties…", "File", Some("⌘D"), "info"),
    command("file.close", "Close tab", "File", Some("⌘W"), "x"),
    command("file.copy_metadata", "Copy document metadata as JSON", "File", None, "braces"),
    command("edit.undo", "Undo", "Edit", Some("⌘Z"), "rotate-ccw"),
    command("edit.redo", "Redo", "Edit", Some("⇧⌘Z"), "rotate-cw"),
    command("view.home", "Go to Home", "View", None, "house"),
    command("view.zoom_in", "Zoom in", "View", Some("⌘+"), "zoom-in"),
    command("view.zoom_out", "Zoom out", "View", Some("⌘−"), "zoom-out"),
    command("view.actual", "Actual size", "View", Some("⌘1"), "maximize"),
    command("view.fit_page", "Zoom to page level", "View", Some("⌘0"), "columns-2"),
    command("view.fit_width", "Fit to width", "View", Some("⌘2"), "maximize"),
    command("view.fit_height", "Fit to height", "View", None, "columns-2"),
    command("view.rotate_cw", "Rotate view clockwise", "View", Some("⇧⌘+"), "rotate-cw"),
    command("view.two_up", "Two-page view", "View", None, "columns-2"),
    command("view.single", "Single page", "View", None, "square"),
    command("view.find", "Find…", "View", Some("⌘F"), "search"),
    command("view.organize", "Organize pages", "View", None, "layout-grid"),
    command("view.palette", "Search tools and commands", "View", Some("⌘K"), "search"),
    command("view.theme", "Toggle light and dark theme", "View", None, "moon"),
    command("comment.list", "Show the comments panel", "View", None, "message-square-text"),
    command("help.shortcuts", "Keyboard shortcuts", "Help", Some("⌘/"), "circle-help"),
    command("help.discord", "Join our Discord", "Help", None, "messages-square"),
    command("help.about", "About PrintCraft", "Help", None, "info"),
];

pub fn command_by_id(id: &str) -> Option<&'static Command> {
    COMMANDS.iter().find(|c| c.id == id)
}

/// The menus of the mode bar, in order.
pub const MENUS: [&str; 5] = ["File", "Edit", "View", "Help", "Tools"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_group_ids_are_unique_and_populated() {
        let mut ids: Vec<&str> = TOOL_GROUPS.iter().map(|g| g.id).collect();
        ids.sort_unstable();
        let count = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), count, "duplicate tool-group id");

        for group in TOOL_GROUPS {
            assert!(!group.sections.is_empty(), "{} has no sections", group.id);
            assert!(group.item_count() > 0, "{} has no items", group.id);
            assert!(ToolGroup::find(group.id).is_some(), "{} does not round-trip", group.id);
        }
    }

    #[test]
    fn command_ids_are_unique_and_addressable() {
        let mut ids: Vec<&str> = COMMANDS.iter().map(|c| c.id).collect();
        ids.sort_unstable();
        let count = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), count, "duplicate command id");

        for command in COMMANDS {
            assert!(command_by_id(command.id).is_some(), "{} does not round-trip", command.id);
            assert!(MENUS.contains(&command.menu), "{} has unknown menu {}", command.id, command.menu);
        }
    }

    #[test]
    fn availability_is_reported_honestly() {
        assert!(Availability::Ready.is_ready());
        assert_eq!(Availability::Ready.badge(), None);
        assert_eq!(Availability::Planned("M7").badge(), Some("M7"));
        assert!(!Availability::Planned("M7").is_ready());
        assert_eq!(Availability::Provider.badge(), Some("Provider"));
    }
}
