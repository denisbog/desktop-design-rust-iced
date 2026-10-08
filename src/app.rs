//! Application state, messages and update logic.
//!
//! The state mirrors PrintCraft's `PrintCraftApp`: a set of open documents with one active tab, a
//! mode, a left panel, an optional right panel, and transient surfaces (palette, dialog, toast).

use std::time::Duration;

use iced::keyboard::{self, key};
use iced::{Subscription, Task};

use crate::catalog;
use crate::theme::ThemeKind;
use crate::ui;

// ---------------------------------------------------------------------------------------------
// Workspace vocabulary (PrintCraft's Mode / LeftPanel / RightPanel, plus view state)
// ---------------------------------------------------------------------------------------------

/// Top-level workspace modes (Acrobat's mode bar).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    AllTools,
    Read,
    Edit,
    Convert,
    Sign,
}

impl Mode {
    pub const ALL: [Mode; 5] = [Mode::AllTools, Mode::Read, Mode::Edit, Mode::Convert, Mode::Sign];

    pub fn label(self) -> &'static str {
        match self {
            Mode::AllTools => "All tools",
            Mode::Read => "Read",
            Mode::Edit => "Edit",
            Mode::Convert => "Convert",
            Mode::Sign => "E-Sign",
        }
    }

    /// The left panel a mode switches to when it is chosen.
    pub fn left(self) -> LeftPanel {
        match self {
            Mode::AllTools => LeftPanel::AllTools,
            Mode::Edit => LeftPanel::Tool("edit"),
            Mode::Convert => LeftPanel::Tool("export"),
            Mode::Sign => LeftPanel::Tool("fill_sign"),
            Mode::Read => LeftPanel::AllTools,
        }
    }
}

/// What the left panel shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeftPanel {
    AllTools,
    Tool(&'static str),
}

/// Right-hand panels, opened from the rail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RightPanel {
    Comments,
    Bookmarks,
    Pages,
    Fields,
    Layers,
    Attachments,
    Signatures,
}

impl RightPanel {
    pub fn icon(self) -> &'static str {
        match self {
            RightPanel::Comments => "message-square-text",
            RightPanel::Bookmarks => "bookmark",
            RightPanel::Pages => "files",
            RightPanel::Fields => "text-cursor-input",
            RightPanel::Layers => "layers",
            RightPanel::Attachments => "paperclip",
            RightPanel::Signatures => "signature",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            RightPanel::Comments => "Comments",
            RightPanel::Bookmarks => "Bookmarks",
            RightPanel::Pages => "Page thumbnails",
            RightPanel::Fields => "Form fields",
            RightPanel::Layers => "Layers",
            RightPanel::Attachments => "Attachments",
            RightPanel::Signatures => "Signatures",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fit {
    Page,
    Width,
    Height,
    Actual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageLayout {
    Continuous,
    Single,
    TwoUp,
}

/// Modal surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dialog {
    Properties(PropsTab),
    Shortcuts,
    About,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropsTab {
    Description,
    InitialView,
    Security,
    Fonts,
}

impl PropsTab {
    pub const ALL: [PropsTab; 4] = [PropsTab::Description, PropsTab::InitialView, PropsTab::Security, PropsTab::Fonts];

    pub fn label(self) -> &'static str {
        match self {
            PropsTab::Description => "Description",
            PropsTab::InitialView => "Initial View",
            PropsTab::Security => "Security",
            PropsTab::Fonts => "Fonts",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Document model
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Comment {
    pub author: &'static str,
    pub initials: &'static str,
    pub page: usize,
    pub time: &'static str,
    pub body: String,
    pub replies: Vec<(String, &'static str)>,
    pub resolved: bool,
}

#[derive(Clone, Debug)]
pub struct OutlineItem {
    pub title: String,
    pub page: usize,
    pub children: Vec<OutlineItem>,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: &'static str,
    pub kind: &'static str,
    pub value: String,
    pub page: usize,
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub name: &'static str,
    pub visible: bool,
    pub locked: bool,
}

#[derive(Clone, Debug)]
pub struct Attachment {
    pub name: &'static str,
    pub size: &'static str,
}

/// One open document (tab).
#[derive(Clone, Debug)]
pub struct Doc {
    pub name: String,
    pub path: Option<String>,
    pub pages: usize,
    pub dirty: bool,
    pub size: String,
    pub comments: Vec<Comment>,
    pub outline: Vec<OutlineItem>,
    pub fields: Vec<Field>,
    pub layers: Vec<Layer>,
    pub attachments: Vec<Attachment>,
    pub signed: bool,
}

impl Doc {
    /// The label shown on the tab and in the title bar.
    pub fn display_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug)]
pub struct RecentFile {
    pub name: String,
    pub path: String,
    pub pages: usize,
    pub size: String,
}

// ---------------------------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------------------------

pub struct App {
    pub theme: ThemeKind,
    pub mode: Mode,
    pub left_open: bool,
    pub left: LeftPanel,
    pub right: Option<RightPanel>,
    /// Index into `docs`; `None` shows the Home tab.
    pub active: Option<usize>,
    pub docs: Vec<Doc>,
    pub recent: Vec<RecentFile>,

    // Transient surfaces.
    pub palette_open: bool,
    pub palette_query: String,
    pub dialog: Option<Dialog>,
    pub toast: Option<(String, f64)>,
    pub menu: Option<&'static str>,

    // Find bar.
    pub find_open: bool,
    pub find_query: String,

    // View state of the active document.
    pub page: usize,
    pub zoom: f32,
    pub fit: Fit,
    pub layout: PageLayout,
    pub page_input: String,

    // Organize pages.
    pub organizing: bool,
    pub selected_pages: Vec<usize>,

    // Comments.
    pub comment_draft: String,
    /// Seconds since start, advanced by the tick subscription; drives toast expiry.
    pub clock: f64,
}

impl Default for App {
    fn default() -> Self {
        let showcase = Doc {
            name: "PrintCraft Showcase.pdf".into(),
            path: Some("/Users/you/Documents/PrintCraft Showcase.pdf".into()),
            pages: 13,
            dirty: false,
            size: "4.2 MB".into(),
            comments: vec![
                Comment {
                    author: "Ada Lovelace",
                    initials: "AL",
                    page: 2,
                    time: "10:24",
                    body: "The drop cap reads too loose at 125% — can we tighten the leading?".into(),
                    replies: vec![("Grace Hopper".into(), "Agreed; I'll set it to 0.92.".into())],
                    resolved: false,
                },
                Comment {
                    author: "Grace Hopper",
                    initials: "GH",
                    page: 5,
                    time: "09:02",
                    body: "The syntax colouring on the listing uses the dark-theme palette on a white page.".into(),
                    replies: vec![],
                    resolved: true,
                },
            ],
            outline: vec![
                OutlineItem {
                    title: "Cover".into(),
                    page: 1,
                    children: vec![],
                },
                OutlineItem {
                    title: "Foreword".into(),
                    page: 2,
                    children: vec![],
                },
                OutlineItem {
                    title: "Setting Text".into(),
                    page: 3,
                    children: vec![
                        OutlineItem { title: "Kerning and ligatures".into(), page: 3, children: vec![] },
                        OutlineItem { title: "Vertical CJK".into(), page: 4, children: vec![] },
                    ],
                },
                OutlineItem {
                    title: "Code and Images".into(),
                    page: 8,
                    children: vec![
                        OutlineItem { title: "Syntax listings".into(), page: 8, children: vec![] },
                        OutlineItem { title: "Fractals".into(), page: 10, children: vec![] },
                    ],
                },
                OutlineItem {
                    title: "Scripts of the World".into(),
                    page: 12,
                    children: vec![],
                },
            ],
            fields: vec![
                Field { name: "Name", kind: "Text field", value: "Ada Lovelace".into(), page: 6 },
                Field { name: "Subscribe", kind: "Checkbox", value: "Off".into(), page: 6 },
                Field { name: "Country", kind: "Drop-down", value: "United Kingdom".into(), page: 6 },
                Field { name: "Signature", kind: "Signature", value: "Not signed".into(), page: 7 },
            ],
            layers: vec![
                Layer { name: "Background", visible: true, locked: true },
                Layer { name: "Body text", visible: true, locked: false },
                Layer { name: "Watermark", visible: false, locked: false },
            ],
            attachments: vec![
                Attachment { name: "showcase-data.csv", size: "12 KB" },
                Attachment { name: "brand-guide.pdf", size: "820 KB" },
            ],
            signed: false,
        };

        Self {
            theme: ThemeKind::Light,
            mode: Mode::AllTools,
            left_open: true,
            left: LeftPanel::AllTools,
            right: Some(RightPanel::Comments),
            active: Some(0),
            docs: vec![showcase],
            recent: vec![
                RecentFile {
                    name: "PrintCraft Showcase.pdf".into(),
                    path: "~/Documents/PrintCraft Showcase.pdf".into(),
                    pages: 13,
                    size: "4.2 MB".into(),
                },
                RecentFile {
                    name: "Q3 Report.pdf".into(),
                    path: "~/Documents/Work/Q3 Report.pdf".into(),
                    pages: 42,
                    size: "1.8 MB".into(),
                },
                RecentFile {
                    name: "Invoice-0042.pdf".into(),
                    path: "~/Downloads/Invoice-0042.pdf".into(),
                    pages: 3,
                    size: "96 KB".into(),
                },
            ],
            palette_open: false,
            palette_query: String::new(),
            dialog: None,
            toast: None,
            menu: None,
            find_open: false,
            find_query: String::new(),
            page: 0,
            zoom: 0.72,
            fit: Fit::Width,
            layout: PageLayout::Continuous,
            page_input: String::new(),
            organizing: false,
            selected_pages: vec![],
            comment_draft: String::new(),
            clock: 0.0,
        }
    }
}

impl App {
    pub fn tokens(&self) -> crate::theme::Tokens {
        crate::theme::Tokens::for_kind(self.theme)
    }

    pub fn doc(&self) -> Option<&Doc> {
        self.active.and_then(|i| self.docs.get(i))
    }

    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.active.and_then(|i| self.docs.get_mut(i))
    }

    pub fn notify(&mut self, message: impl Into<String>) {
        self.toast = Some((message.into(), self.clock));
    }

    /// How many times the find query occurs on the active document (a stand-in for real search).
    pub fn match_count(&self) -> usize {
        if self.find_query.trim().is_empty() {
            0
        } else {
            // Deterministic, so the UI reads the same every frame.
            (self.find_query.len() * 3 + self.find_query.bytes().map(|b| b as usize).sum::<usize>()) % 17 + 1
        }
    }

    /// The zoom implied by the current fit mode.
    pub fn effective_zoom(&self) -> f32 {
        match self.fit {
            Fit::Page => 0.62,
            Fit::Width => 0.72,
            Fit::Height => 0.58,
            Fit::Actual => 1.0,
        }
    }

    /// Ask the OS for a PDF and open it as a new tab (the rfd integration PrintCraft uses).
    fn add_opened_document(&mut self, path: &std::path::Path) {
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "Document.pdf".into());
        let size = std::fs::metadata(path).map(|m| human_size(m.len())).unwrap_or_else(|_| "—".into());
        let display_path = path.to_string_lossy().into_owned();

        self.docs.push(Doc {
            name: name.clone(),
            path: Some(display_path.clone()),
            pages: 1,
            dirty: false,
            size: size.clone(),
            comments: vec![],
            outline: vec![OutlineItem { title: name.clone(), page: 1, children: vec![] }],
            fields: vec![],
            layers: vec![Layer { name: "Layer 1", visible: true, locked: false }],
            attachments: vec![],
            signed: false,
        });
        self.active = Some(self.docs.len() - 1);
        self.page = 0;
        self.selected_pages.clear();
        self.recent.insert(
            0,
            RecentFile { name: name.clone(), path: display_path, pages: 1, size },
        );
        self.notify(format!("Opened {name}"));
    }

    /// The document's metadata as JSON (serde + serde_json), for the "Copy metadata" command.
    fn metadata_json(&self) -> String {
        #[derive(serde::Serialize)]
        struct Metadata<'a> {
            file: &'a str,
            pages: usize,
            size: &'a str,
            edited: bool,
            comments: usize,
            bookmarks: usize,
            form_fields: usize,
            layers: usize,
            attachments: usize,
        }

        let Some(doc) = self.doc() else {
            return serde_json::json!({ "error": "no active document" }).to_string();
        };
        let metadata = Metadata {
            file: doc.display_name(),
            pages: doc.pages,
            size: &doc.size,
            edited: doc.dirty,
            comments: doc.comments.len(),
            bookmarks: doc.outline.len(),
            form_fields: doc.fields.len(),
            layers: doc.layers.len(),
            attachments: doc.attachments.len(),
        };
        serde_json::to_string_pretty(&metadata).unwrap_or_else(|_| "{}".into())
    }

    fn select_tab(&mut self, index: Option<usize>) {
        self.active = index;
        self.page = 0;
        self.selected_pages.clear();
        if index.is_none() {
            self.organizing = false;
        }
    }

    fn add_demo_document(&mut self) {
        let n = self.docs.len();
        self.docs.push(Doc {
            name: format!("Untitled {n}.pdf"),
            path: None,
            pages: 1,
            dirty: true,
            size: "—".into(),
            comments: vec![],
            outline: vec![OutlineItem { title: "Untitled".into(), page: 1, children: vec![] }],
            fields: vec![],
            layers: vec![Layer { name: "Layer 1", visible: true, locked: false }],
            attachments: vec![],
            signed: false,
        });
        self.active = Some(n);
        self.page = 0;
        self.notify("Created a new blank document");
    }

    fn close_tab(&mut self, index: usize) {
        if index >= self.docs.len() {
            return;
        }
        let had_unsaved = self.docs[index].dirty;
        self.docs.remove(index);
        self.active = match self.active {
            Some(current) if current == index => {
                if self.docs.is_empty() {
                    None
                } else {
                    Some(current.min(self.docs.len() - 1))
                }
            }
            Some(current) if current > index => Some(current - 1),
            other => other.filter(|i| *i < self.docs.len()),
        };
        if had_unsaved {
            self.notify("Closed a document with unsaved changes (discarded)");
        } else {
            self.notify("Tab closed");
        }
    }

    fn run_command(&mut self, command: &str) -> Task<Message> {
        match command {
            "file.open" => return update(self, Message::OpenRequested),
            "file.copy_metadata" => {
                let json = self.metadata_json();
                self.notify("Copied the document metadata as JSON");
                return iced::clipboard::write(json);
            }
            "file.new" => self.add_demo_document(),
            "file.save" => {
                if let Some(doc) = self.doc_mut() {
                    doc.dirty = false;
                }
                self.notify("Saved incrementally (original bytes untouched)");
            }
            "file.print" => self.notify("Print dialog is not part of this design demo"),
            "file.properties" => self.dialog = Some(Dialog::Properties(PropsTab::Description)),
            "file.close" => {
                if let Some(i) = self.active {
                    self.close_tab(i);
                }
            }
            "edit.undo" => self.notify("Undo"),
            "edit.redo" => self.notify("Redo"),
            "view.home" => self.select_tab(None),
            "view.zoom_in" => self.zoom_by(1.15),
            "view.zoom_out" => self.zoom_by(1.0 / 1.15),
            "view.actual" => {
                self.fit = Fit::Actual;
                self.zoom = 1.0;
            }
            "view.fit_page" => self.fit = Fit::Page,
            "view.fit_width" => self.fit = Fit::Width,
            "view.fit_height" => self.fit = Fit::Height,
            "view.rotate_cw" => self.notify("Rotated the view clockwise"),
            "view.two_up" => self.layout = PageLayout::TwoUp,
            "view.single" => self.layout = PageLayout::Single,
            "view.find" => self.find_open = true,
            "view.organize" => {
                self.organizing = true;
                self.selected_pages.clear();
            }
            "view.palette" => self.palette_open = true,
            "view.theme" => self.toggle_theme(),
            "comment.list" => {
                self.right = Some(RightPanel::Comments);
            }
            "help.shortcuts" => self.dialog = Some(Dialog::Shortcuts),
            "help.about" => self.dialog = Some(Dialog::About),
            "help.discord" => self.notify("Opening discord.gg/artcraft"),
            "page.organize" => {
                self.organizing = true;
                self.selected_pages.clear();
            }
            other => {
                let tool = catalog::TOOL_GROUPS.iter().find(|g| g.sections.iter().any(|s| s.items.iter().any(|i| i.command == other)));
                if let Some(group) = tool {
                    self.left_open = true;
                    self.left = LeftPanel::Tool(group.id);
                } else if let Some(cmd) = catalog::command_by_id(other) {
                    self.notify(format!("“{}” is a placeholder in this demo", cmd.label));
                } else {
                    self.notify(format!("“{other}” is not wired up in this demo"));
                }
            }
        }
        Task::none()
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            ThemeKind::Light => ThemeKind::Dark,
            ThemeKind::Dark => ThemeKind::Light,
        };
    }

    fn zoom_by(&mut self, factor: f32) {
        self.fit = Fit::Actual;
        self.zoom = (self.zoom * factor).clamp(0.15, 6.0);
    }

    fn step_page(&mut self, forward: bool) {
        let pages = self.doc().map(|d| d.pages).unwrap_or(1);
        self.page = if forward {
            (self.page + 1).min(pages.saturating_sub(1))
        } else {
            self.page.saturating_sub(1)
        };
        self.page_input.clear();
    }
}

// ---------------------------------------------------------------------------------------------
// Messages and update
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Message {
    // Chrome.
    ToggleTheme,
    SetMode(Mode),
    OpenLeft(LeftPanel),
    ToggleLeft,
    ToggleRight(RightPanel),
    SelectTab(Option<usize>),
    CloseTab(usize),
    /// Ask the OS for a PDF (rfd) and open it.
    OpenRequested,
    /// The file the OS dialog returned, if any.
    OpenPicked(Option<std::path::PathBuf>),
    OpenDialog(Dialog),
    CloseDialog,
    ToggleMenu(&'static str),
    CloseMenu,
    SetPropsTab(PropsTab),

    // Command palette.
    PaletteOpen,
    PaletteClose,
    PaletteQuery(String),
    /// Enter in the palette runs the top result.
    PaletteSubmit,
    PalettePick(String),

    // Find.
    FindToggle,
    FindQuery(String),
    FindNext,
    FindPrev,

    // View.
    ZoomIn,
    ZoomOut,
    SetFit(Fit),
    NextPage,
    PrevPage,
    PageInput(String),
    PageSubmit,
    ToggleOrganize,
    TogglePageSelected(usize),
    GoToPage(usize),
    SelectAllPages,
    ClearSelection,
    RotateSelected,
    DeleteSelected,

    // Right panel.
    CommentDraft(String),
    AddComment,
    ToggleResolve(usize),
    ToggleLayer(usize),

    // Transient.
    Notify(String),
    Tick(iced::time::Instant),

    // Keyboard.
    Key(keyboard::Event),
}

pub fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::ToggleTheme => app.toggle_theme(),
        Message::SetMode(mode) => {
            app.mode = mode;
            app.left_open = true;
            app.left = mode.left();
            if mode == Mode::Read {
                app.organizing = false;
            }
        }
        Message::OpenLeft(panel) => {
            app.left = panel;
            app.left_open = true;
        }
        Message::ToggleLeft => app.left_open = !app.left_open,
        Message::ToggleRight(panel) => {
            app.right = if app.right == Some(panel) { None } else { Some(panel) };
        }
        Message::SelectTab(index) => app.select_tab(index),
        Message::CloseTab(index) => app.close_tab(index),
        Message::OpenDialog(dialog) => {
            app.dialog = Some(dialog);
            app.menu = None;
        }
        Message::CloseDialog => app.dialog = None,
        Message::ToggleMenu(menu) => {
            app.menu = if app.menu == Some(menu) { None } else { Some(menu) };
        }
        Message::CloseMenu => app.menu = None,
        Message::SetPropsTab(tab) => app.dialog = Some(Dialog::Properties(tab)),

        Message::PaletteOpen => {
            app.palette_open = true;
            app.palette_query.clear();
            return iced::widget::operation::focus(ui::overlays::PALETTE_INPUT);
        }
        Message::PaletteClose => {
            app.palette_open = false;
            app.palette_query.clear();
        }
        Message::PaletteQuery(query) => app.palette_query = query,
        Message::PaletteSubmit => {
            if let Some(hit) = ui::overlays::hits(&app.palette_query).first().cloned() {
                let message = ui::overlays::hit_message(&hit);
                return update(app, message);
            }
        }
        Message::PalettePick(command) => {
            app.palette_open = false;
            app.palette_query.clear();
            return app.run_command(&command);
        }
        Message::OpenRequested => {
            return Task::perform(
                async { rfd::FileDialog::new().add_filter("PDF documents", &["pdf"]).pick_file() },
                Message::OpenPicked,
            );
        }
        Message::OpenPicked(Some(path)) => app.add_opened_document(&path),
        Message::OpenPicked(None) => {}

        Message::FindToggle => {
            app.find_open = !app.find_open;
            if !app.find_open {
                app.find_query.clear();
            }
            return iced::widget::operation::focus(ui::canvas::FIND_INPUT);
        }
        Message::FindQuery(query) => app.find_query = query,
        Message::FindNext => app.notify("Jumped to the next match"),
        Message::FindPrev => app.notify("Jumped to the previous match"),

        Message::ZoomIn => app.zoom_by(1.15),
        Message::ZoomOut => app.zoom_by(1.0 / 1.15),
        Message::SetFit(fit) => {
            app.fit = fit;
            app.zoom = app.effective_zoom();
        }
        Message::NextPage => app.step_page(true),
        Message::PrevPage => app.step_page(false),
        Message::PageInput(text) => app.page_input = text,
        Message::PageSubmit => {
            if let Ok(typed) = app.page_input.trim().parse::<usize>() {
                let pages = app.doc().map(|d| d.pages).unwrap_or(1);
                if typed >= 1 && typed <= pages {
                    app.page = typed - 1;
                }
            }
            app.page_input.clear();
        }
        Message::GoToPage(index) => {
            let pages = app.doc().map(|d| d.pages).unwrap_or(1);
            app.page = index.min(pages.saturating_sub(1));
        }
        Message::ToggleOrganize => {
            app.organizing = !app.organizing;
            app.selected_pages.clear();
        }
        Message::TogglePageSelected(index) => {
            if let Some(pos) = app.selected_pages.iter().position(|p| *p == index) {
                app.selected_pages.remove(pos);
            } else {
                app.selected_pages.push(index);
            }
        }
        Message::SelectAllPages => {
            let pages = app.doc().map(|d| d.pages).unwrap_or(0);
            app.selected_pages = (0..pages).collect();
        }
        Message::ClearSelection => app.selected_pages.clear(),
        Message::RotateSelected => {
            let n = app.selected_pages.len();
            if n == 0 {
                app.notify("Select pages first");
            } else {
                if let Some(doc) = app.doc_mut() {
                    doc.dirty = true;
                }
                app.notify(format!("Rotate pages ({n})"));
            }
        }
        Message::DeleteSelected => {
            let mut selected = app.selected_pages.clone();
            if selected.is_empty() {
                app.notify("Select pages first");
            } else {
                selected.sort_unstable();
                if let Some(doc) = app.doc_mut() {
                    doc.pages = doc.pages.saturating_sub(selected.len()).max(1);
                    doc.dirty = true;
                    doc.comments.retain(|c| !selected.contains(&(c.page - 1)));
                }
                app.page = app.page.min(app.doc().map(|d| d.pages).unwrap_or(1).saturating_sub(1));
                app.selected_pages.clear();
                app.notify("Deleted the selected pages (undo to restore)");
            }
        }

        Message::CommentDraft(text) => app.comment_draft = text,
        Message::AddComment => {
            let body = app.comment_draft.trim().to_string();
            if body.is_empty() {
                return Task::none();
            }
            let page = app.page + 1;
            if let Some(doc) = app.doc_mut() {
                doc.comments.insert(
                    0,
                    Comment {
                        author: "You",
                        initials: "YO",
                        page,
                        time: "now",
                        body,
                        replies: vec![],
                        resolved: false,
                    },
                );
                doc.dirty = true;
            }
            app.comment_draft.clear();
            app.right = Some(RightPanel::Comments);
            app.notify("Comment added");
        }
        Message::ToggleResolve(index) => {
            if let Some(doc) = app.doc_mut() {
                if let Some(comment) = doc.comments.get_mut(index) {
                    comment.resolved = !comment.resolved;
                }
            }
        }
        Message::ToggleLayer(index) => {
            if let Some(doc) = app.doc_mut() {
                if let Some(layer) = doc.layers.get_mut(index) {
                    layer.visible = !layer.visible;
                }
            }
        }

        Message::Notify(text) => app.notify(text),
        Message::Tick(_now) => {
            app.clock += 0.5;
            if let Some((_, start)) = app.toast {
                if app.clock - start > 3.5 {
                    app.toast = None;
                }
            }
        }

        Message::Key(event) => {
            if let keyboard::Event::KeyPressed { key, modifiers, .. } = event {
                match key.as_ref() {
                    key::Key::Named(key::Named::Escape) => {
                        if app.palette_open {
                            app.palette_open = false;
                            app.palette_query.clear();
                        } else if app.dialog.is_some() {
                            app.dialog = None;
                        } else if app.menu.is_some() {
                            app.menu = None;
                        } else if app.find_open {
                            app.find_open = false;
                            app.find_query.clear();
                        }
                    }
                    key::Key::Character("k") if modifiers.command() => {
                        app.palette_open = true;
                        app.palette_query.clear();
                        return iced::widget::operation::focus(ui::overlays::PALETTE_INPUT);
                    }
                    key::Key::Character("f") if modifiers.command() => {
                        app.find_open = true;
                        return iced::widget::operation::focus(ui::canvas::FIND_INPUT);
                    }
                    _ => {}
                }
            }
        }
    }
    Task::none()
}

/// The one always-on subscription: keyboard shortcuts, plus a slow tick for the toast.
pub fn subscription(_app: &App) -> Subscription<Message> {
    let keys = keyboard::listen().map(Message::Key);
    let ticks = iced::time::every(Duration::from_millis(500)).map(Message::Tick);
    Subscription::batch([keys, ticks])
}

/// Format a byte count for the recent-files list ("4.2 MB", "96 KB").
fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    let bytes = bytes as f64;
    if bytes >= MB {
        format!("{:.1} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes / KB)
    } else {
        format!("{bytes:.0} B")
    }
}
