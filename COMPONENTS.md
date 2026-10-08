# PrintCraft (iced) — UI component reference

Every component in the demo, in the order it appears on screen, with what it does, where it lives,
and which PrintCraft (egui) module it mirrors.

The window is one `iced::Element` tree rebuilt each frame from `App`. The root lives in
[`src/view.rs`](src/view.rs); everything below is a plain function that returns an `Element`, so the
composition reads exactly like the egui original.

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ tab_strip            Home │ tabs │ Open │ … │ theme · help · Discord         │ 38 px
├──────────────────────────────────────────────────────────────────────────────┤
│ mode_bar             Menu │ All tools · Read · Edit · Convert · E-Sign │ ⛭ ⛁ ⓘ │ ⌘K │ 48 px
├───────────┬──────────────────────────────────────────────┬───────────┬───────┤
│ left      │ canvas                                       │ right     │ rail  │
│ 264 px    │ (home · reader · organize)                   │ 300 px    │ 48 px │
└───────────┴──────────────────────────────────────────────┴───────────┴───────┘
floating layers: menu_dropdown · palette · dialog · toast
```

---

## 1. Foundation

### 1.1 `theme::Tokens` — design tokens
**File:** `src/theme.rs` · **PrintCraft:** `crates/ui-egui/src/theme.rs`

The single source of colour truth. `Tokens::for_kind(ThemeKind::{Light,Dark})` returns the whole set
and every widget takes a `Tokens` by value (`Copy`), so no widget hard-codes a colour.

| Token | Purpose (light value) |
| --- | --- |
| `titlebar` | window/tab strip background (`#E9E9EB`) |
| `chrome` | mode bar, rails, active tab (`#FFFFFF`) |
| `panel` | left/right panel bodies and headers (`#FFFFFF`) |
| `pasteboard` | the workspace behind a page (`#F1F1F3`) |
| `card` | cards, popups, dialogs, the page (`#FFFFFF`) |
| `border` / `divider` | field outlines / hairline separators |
| `text` / `text_muted` / `text_faint` | three-level text hierarchy, all ≥ 4.5:1 on `chrome` |
| `icon` | default icon tint |
| `hover` / `pressed` | interaction feedback fills |
| `selected` | row/page selection fill |
| `accent` / `accent_text` / `accent_soft` | the one blue accent: fill, readable text, tinted background |
| `field` | text input background |
| `page_shadow` | the shadow an open page casts on the pasteboard |
| `radius` | 6 px corner radius used by buttons and icon buttons |

`Tokens::palette()` + `to_theme()` derive the iced `Theme` (background, text, primary, success,
warning, danger) so built-in widget defaults outside our control stay coherent; `Tokens::dark()`
answers "is this the dark theme".

**Dark Gray rule:** pages are drawn on `Color::WHITE` paper in *both* themes. The ink colours in the
document area are fixed constants (`INK`, `INK_MUTED`, `INK_FAINT`, `ACCENT`, `PAPER` in
`src/ui/canvas.rs`), deliberately independent of the tokens.

### 1.2 Fonts
**File:** `src/theme.rs` · **PrintCraft:** `theme::install_fonts`, `theme::regular/medium/semibold`

The four OFL faces PrintCraft bundles are embedded with `include_bytes!` (`theme::fonts()`):
Inter Regular/Medium/SemiBold and JetBrains Mono Regular. `main.rs` registers each with
`.font(...)` and makes Inter Regular the app default with `.default_font(theme::regular())`.

- `theme::regular()` → Inter 400 (body)
- `theme::medium()` → Inter 500 (labels, buttons, tab names)
- `theme::semibold()` → Inter 600 (headings)
- `theme::mono()` → JetBrains Mono (numeric readouts, such as the zoom level)

### 1.3 Icon set
**File:** `src/icons.rs` + `build.rs` · **PrintCraft:** `icon_data.rs` + `icons.rs`

`build.rs` walks `assets/icons/*.svg` (the Lucide set) and writes `$OUT_DIR/icons.rs`, a
`&[(name, bytes)]` table. `icons::handle(name)` returns an `svg::Handle::from_memory`, falling back
to a neutral `square` so a typo can never panic. `icons::icon::<M>(name, size, tint)` renders it at
an exact size with a colour filter (`.style(|_,_| svg::Style { color: Some(tint) })`), which is how
one monochrome file serves every tint in the design. `icons::exists(name)` backs the catalogue test.
The `#[test] every_catalog_icon_exists` fails the build if any catalogue row names a missing icon.

### 1.4 Catalogue
**File:** `src/catalog.rs` · **PrintCraft:** `crates/engine/src/catalog.rs` + `commands.rs`

Pure data — no UI. The left panel, the palette, the menus and the home screen all iterate it.

- `Availability::{Ready, Planned(&str), Provider}` — `is_ready()`, `badge()` ("M7", "Provider"),
  `note()` ("Works in this build"). Planned and provider rows stay visible but are disabled, so the
  UI never lies about what works.
- `ToolItem { label, icon, command, availability }` — one actionable row.
- `ToolSection { title, items }` — a titled group ("Markup", "Drawing", "Review").
- `ToolGroup { id, label, icon, hue, availability, sections }` — a tool family: `blurb()` (first three
  item labels, joined with "·"), `item_count()`, `find(id)`.
- `Command { id, label, menu, shortcut, icon }` — the registry the palette and menus dispatch by id.
  `COMMANDS`, `command_by_id(id)`, `MENUS`.
- 14 tool groups, ~90 items and 26 commands, all with real Lucide icons.

---

## 2. Widgets
**File:** `src/widgets.rs` · **PrintCraft:** `crates/ui-egui/src/widgets.rs`

Reusable constructors built on the tokens. Hover/press feedback comes from iced's `button::Status`
rather than manual hit-testing, so everything stays keyboard- and accessibility-aware.

| Widget | Functionality |
| --- | --- |
| `mode_tab(label, active, t, on_press)` | A mode-bar tab: 48 px tall, text with a 3 px underline when active, subtle hover fill otherwise. Used by the mode bar. |
| `mode_tab_small(label, active, t, on_press)` | The compact variant used for the menu-group row and the properties tabs; active state fills with `accent_soft`. |
| `pill_button(label, primary, t, on_press)` | Rounded pill. `primary` fills with the accent and white text; otherwise an outlined card. Primary actions only (Comment, Delete). |
| `ghost_button(icon, label, t, on_press)` | Icon + label, transparent until hovered. Used by "Menu", "Open", "Discord". |
| `icon_button(name, box_size, selected, enabled, t, on_press)` | Square icon button of exactly `box_size` px. The glyph is `box_size * 0.42` (min 11 px) and the box is built from *padding*, not from `.width(Fixed)` — see the sizing note below. `selected` paints `accent_soft` + `accent_text`; `enabled=false` greys the icon and removes the press handler (`on_press_maybe`) so a dead control is genuinely inert. |
| `search_box(placeholder, width, t, on_press)` | A search-field *lookalike* (icon, placeholder, right-aligned `⌘K`) that opens the command palette. Not an input, so it never steals focus. |
| `menu_item(label, shortcut, enabled, t, on_press)` | A menu row: label left, shortcut right, hover fill, disabled text when `enabled=false`. |
| `section_title(title, t)` | All-caps 10.5 px semibold, letter-spaced, faint. Panel section heading. |
| `icon_pill(icon, label, primary, t, on_press)` | Pill with a leading icon (fits the primary/secondary pair used by the community card). |
| `badge(label, fill, fg)` | A small status pill: availability ("M7"), field values, "Esc", "Open"/"Resolved". |
| `v_divider(t)` / `h_divider(t)` | 1 px separators that fill the cross axis. |
| `hspace(px)` / `vspace(px)` | Fixed spacers (iced's `Space` is otherwise awkward inline). |
| `tip(content, label, t)` | Wraps any widget in a 6 px-gap tooltip styled like the chrome. Every icon button in the app is wrapped in one. |
| `card_style(t)` | The shared card surface: `card` fill, 1 px `border`, 12 px radius. |
| `scrolled(content, t)` | The app's only scrollable constructor: a 7 px lane with a 5 px rounded thumb, no rail background, thumb `text_faint` → `text_muted` on hover → `icon` while dragging. Callers chain `.height(...)`. |

### 2.1 Sizing rule: never pin a fixed-size child inside a fixed-size `Button`

iced's `Button` lays its content out with `layout::padded(limits, self.width, self.height, …)`, and
`Limits::width(Length::Fixed(n))` sets **both** `min` and `max` to `n`. `Button` does *not* loosen
those limits for its child (unlike `Container`, which calls `limits.loose()`), and
`Limits::resolve(Length::Fixed(v))` is `v.min(max).max(min)` — so v is clamped *up* to the button
size. An icon written as `button(svg(name, 14)).width(Fixed(34)).height(Fixed(34))` therefore renders
at **34 px**, not 14, and no amount of tuning the `14` has any effect.

`icon_button` avoids this by taking its size from padding:

```rust
let glyph_size = (box_size * 0.42).max(11.0).round();
let inset = ((box_size - glyph_size) / 2.0).max(0.0);
button(icons::icon::<Message>(name, glyph_size, tint)).padding(inset)
```

with `box = glyph_size + 2 * inset`. Icons that sit inside a `Row`/`Column` inside a button are safe,
because both flex containers hand children `Limits::with_compression(Size::ZERO, …)` — a zero minimum
— on their first layout pass.

---

## 3. Window chrome
**File:** `src/ui/chrome.rs` · **PrintCraft:** `chrome::tab_strip`, `chrome::mode_bar`

### 3.1 `tab_strip(app, t)` — 38 px
- **Home button** (`house`): the document tabs' "no tab" state; selected when `app.active.is_none()`.
- **`tab(app, t, index, doc)`** (private), one per open document:
  - file icon (`accent` when active, `text_muted` otherwise), the document name
    (middle-safe truncated at 28 chars via `Doc::display_name()`),
  - **dirty dot** when `doc.dirty` (an 8 px `text`-coloured circle) or an **× close button** when clean,
  - clicking selects the tab; × dispatches `CloseTab(index)` (which discards unsaved changes and says so),
  - the active tab is painted with the `chrome` token so it visually merges with the mode bar.
- **Open** ghost button → `Message::OpenRequested` (a real `rfd` file dialog; the picked PDF is added
  as a new tab with the file's name and size).
- Right side: **theme toggle** (`moon`/`sun`, tooltip names the target theme), **keyboard shortcuts**
  (`circle-help` → Shortcuts dialog) and a **Discord** ghost button (the community link PrintCraft
  keeps in its title bar).

### 3.2 `mode_bar(app, t)` — 48 px
- **Menu** ghost button: opens `menu_dropdown`; toggles closed via `CloseMenu`.
- A 1 px `v_divider`.
- **Mode tabs** (`Mode::ALL`): All tools · Read · Edit · Convert · E-Sign. Selecting a mode also
  chooses its left panel (`Mode::left()`), matching PrintCraft's `app.left = …` switch.
- **Document actions** (right-aligned, disabled when no document is open): `printer` (Print),
  `save` (Save — clears the dirty flag and reports "Saved incrementally"), `info` (Document
  properties dialog).
- **`search_box`**: the ⌘K entry point. It is a styled button, so the palette (not the bar) owns focus.

### 3.3 `menu_dropdown(app, t)`
A 280 px floating card: a row of `mode_tab_small` group tabs (File · Edit · View · Help · Tools) over
a scrollable list of that group's `COMMANDS` as `menu_item` rows. The **Tools** group lists every
`ToolGroup` instead, and toggles the left panel. Choosing a command dispatches `PalettePick(id)`, so
menus and the palette share one dispatch path.

---

## 4. Left panel
**File:** `src/ui/left.rs` · **PrintCraft:** `panels.rs` (`all_tools`, `tool_detail`)

Fixed 264 px, `panel` background. `left::view` switches on `app.left`:

### 4.1 `panel_header(title, back, t)` (private)
40 px header: an optional back chevron (returns to "All tools"), the title, a spacer and a
"Hide panel" button that toggles `app.left_open`.

### 4.2 `all_tools(t)` — `LeftPanel::AllTools`
A scrollable list of every `ToolGroup` as a `tool_row`.

### 4.3 `tool_row(group, t)` (private)
One tool family: the group icon tinted with its own `hue` (Acrobat's colour-coding), the label, a
one-line `blurb()`, an availability badge when not ready, and a chevron. Clicking opens the detail
panel (`OpenLeft(LeftPanel::Tool(id))`).

### 4.4 `tool_detail(group, t)` (private) — `LeftPanel::Tool(id)`
- **Hero block**: the hue-tinted icon in a 38 px rounded tile, the group label with an availability
  badge, and a note line reading `"<note> · <item_count> tools"`.
- **Sections**: for each `ToolSection`, an all-caps `section_title`, then one `item_row` per item.

### 4.5 `item_row(label, icon, availability, t, on_press)` (public)
A 16 px icon, the label, an optional milestone badge. Ready rows are `text`-coloured; planned rows
are `text_faint`. Clicking dispatches the item's command through the same path as the palette.

---

## 5. Right rail
**File:** `src/ui/rail.rs` · **PrintCraft:** `chrome::right_rail`

Fixed 48 px, `chrome` background, 1 px divider on its left edge. `rail::view` renders nothing but the
shell when no document is open.

### 5.1 Panel toggles (`rail_button`)
One `icon_button` per `RightPanel` (Comments, Bookmarks, Pages, Fields, Layers, Attachments,
Signatures), each tooltipped with its title. The selected panel gets `accent_soft`. When a document
actually has content for that panel, a 6 px `accent` dot is stacked in the button's top-right corner
(`stack![button, dot]`) — the "there is something in here" hint from PrintCraft's rail. Clicking
toggles (`ToggleRight`), so clicking the active icon closes the panel.

### 5.2 Page & zoom cluster (bottom)
In PrintCraft's bottom-up order, top to bottom:
1. **Page input** — a 38 px centred `text_input`; type a page number and press Enter
   (`PageSubmit` clamps to `1..=pages`). Its style turns the border `accent` while focused.
2. **Page count** (plain label).
3. **`chevron-up` / `chevron-down`** — previous / next page (`PrevPage`, `NextPage`).
4. **Zoom readout** — `%` in JetBrains Mono, derived from `app.zoom`.
5. **Fit toggle** — `maximize` ⇄ `columns-2`, switches between `Fit::Width` and `Fit::Page`
   (tooltip says which way it will go).
6. **`rotate-cw`** — rotate the view.
7. **`zoom-in` / `zoom-out`** — ×1.15 steps, clamped to 15 %–600 %; any manual zoom switches
   `Fit::Actual`.

---

## 6. Right panel
**File:** `src/ui/right.rs` · **PrintCraft:** `panels.rs::right_panel`

Fixed 300 px. `right::view` switches on `app.right` and falls back to an "open a document" empty
state. Every panel is a projection of the active `Doc` and has an explicit empty state.

### 6.1 `shell(title, panel, body, t)` (private)
The frame: a 40 px header (title + × that closes *this* panel via `ToggleRight(panel)`), a divider and
a scrollable, padded body.

### 6.2 `empty(t, icon, message)` (private)
Centred faint icon over a one-line explanation, used by every panel that can be empty.

### 6.3 Comments
- **Composer**: a `text_input` bound to `comment_draft` (`on_input`, `on_submit` = add) and a primary
  "Comment" pill. Empty submissions are ignored; a submission inserts a `Comment` at the top, marks
  the document dirty, clears the draft and ensures the Comments panel is open.
- **`comment_card(index, comment, t)`** (private): a 28 px circular **`avatar`** with the author's
  initials, author name, `Page N · time`, a right-aligned **Open/Resolved** badge, the body text,
  indented **replies** each with a 3 px quote rule, and a **Resolve/Reopen** toggle plus a "Reply"
  affordance. Threads are per-comment, matching PrintCraft's threaded comment list.

### 6.4 Bookmarks
- **`bookmarks(items, t)`**: an empty state or one `outline_item` per entry.
- **`outline_item(item, depth, t)`** (private): recurses into `children`, indenting 14 px per level,
  showing a file icon, the title, and the page number. Clicking jumps to that page (`GoToPage`).

### 6.5 Page thumbnails
A scrollable column of `canvas::page_thumbnail(index, 108, 140, active, t)` under a "Page N" caption.
The current page is highlighted (`selected` fill + accent border); clicking navigates.

### 6.6 Form fields
One card per `Field`: a cursor icon, the name, `kind · page`, and a badge with the field's current
value. Read-only presentation of the document's form model.

### 6.7 Layers
One row per `Layer`: an `eye`/`eye-off` toggle (`ToggleLayer`) that also dims the label, the layer
name, and a `lock`/`lock-open` glyph. Mirrors PDF optional content.

### 6.8 Attachments
One card per `Attachment`: paperclip, name, size, and a `file-down` button that reports "Saved …".

### 6.9 Signatures
"Not signed" empty state, or a signed note when `doc.signed`.

---

## 7. Document area (canvas)
**File:** `src/ui/canvas.rs` · **PrintCraft:** `canvas.rs`

Fills the remaining width; background is the `pasteboard` token. `canvas::view` picks one of three
bodies: Home (no tab), Organize (grid mode), Reader (default).

### 7.1 `reader(app, t)` (private)
`find_bar` (when open) above a `widgets::scrolled` pasteboard (5 px thumb, no rail — the 10 px
accent-blue default is replaced everywhere by `widgets::scrolled`). The page row is built per `app.layout`:
Continuous/Single show one page, **Two-up** shows the next page beside it with a 24 px gutter. Pages
are centred both axes and padded 28 px, so the pasteboard keeps a generous margin at any zoom.

### 7.2 `page_card(index, zoom, t)` + `page_body(index, zoom, t)` (private)
The simulated page: US Letter (612 × 792 pt) × `zoom`, white paper with a 16 px-blur drop shadow, and
52/54 pt margins. Content is generated from five typographic samples cycled by `index`, and shows the
PrintCraft showcase vocabulary: an eyebrow line with the page number, a hairline rule, a 34 pt
semibold title, a subtitle, body text, a **pull quote** with a 3 px accent rule, a coloured **figure
block** (image + caption on the block), and a footer. Everything scales with `zoom`, which is why the
page stays legible from 15 % to 600 %.

### 7.3 `find_bar(app, t)` (private)
A chrome bar above the pasteboard: a `text_input` (id `FIND_INPUT`, so ⌘F can focus it), a match
counter ("4 of 16", derived by `App::match_count`), `chevron-up`/`chevron-down` for previous/next
match, a hint ("⌘G next · ⇧⌘G previous") and an × that closes find and clears the query.

### 7.4 `organize(app, t)` (private) — Organize pages
- **Toolbar**: title, a live selection count ("3 of 13 pages selected"), and pills:
  Select all · Clear · Rotate · Extract · Insert · **Delete** (primary) · Done. Rotate/Delete operate
  on `selected_pages`; Delete really removes pages from the model and re-counts, with a toast that
  reminds you undo exists (`Message::RotateSelected` / `DeleteSelected`).
- **Grid**: five thumbnails per row; each cell is `organize_cell`.

### 7.5 `organize_cell(index, selected, t)` (private)
A thumbnail plus a page number, a selection glyph (`check-circle-2` when selected, `circle`
otherwise) and a "Selected" badge. Selecting toggles `TogglePageSelected`.

### 7.6 `page_thumbnail(index, width, height, active, t)` (public)
Shared by Organize pages and the Pages panel. Draws paper with grey "text" bars (a title bar, four
body bars), an accent bar and a figure block, all scaled by `width / 612`. `active` swaps the 1 px
`border` for a 2 px accent outline. Cheap enough to render a dozen per frame at 108–120 px wide.

---

## 8. Home
**File:** `src/ui/home.rs` · **PrintCraft:** `home.rs`

Shown in the canvas slot when no tab is active.

- **Header**: "Welcome to PrintCraft" (24 px semibold) and a one-line description.
- **Community card**: the app **`mark`** (an accent rounded square with a white glyph) and a copy
  block, then three `icon_pill`s: Join our Discord (primary), Documentation, Keyboard shortcuts.
- **Recommended tools**: a wrapping row of `tool_card`s for `RECOMMENDED`
  (`organize`, `comment`, `fill_sign`, `edit`, `compress`) plus the `open_card`.
  - **`tool_card(group, t)`** (private): 196 × 116 px, hue-tinted icon, label, `blurb()`, and a
    "Use now" accent link; clicking opens that tool's left panel.
  - **`open_card(t)`** (private): 170 × 116 px dashed-feeling tile with a folder icon and
    "Open file"; clicking starts the `rfd` open flow.
- **Recent**: a `recent_row` per `App::recent` entry.
  - **`recent_row(name, path, pages, size, t)`** (private): red PDF glyph, name over path, and a
    right-aligned "13 pages · 4.2 MB"; clicking opens the file dialog.
- **Privacy** section: the offline/no-telemetry statement.

---

## 9. Floating layers
**File:** `src/ui/overlays.rs` · **PrintCraft:** `palette.rs`, `dialogs.rs`, `widgets::toast`

Each layer is pushed onto the root `Stack`, so it floats over the chrome without reflowing it.

### 9.1 Command palette
- **`score(hay, needle)`** (private): case-insensitive substring first, then a subsequence fallback —
  the same forgiving matcher PrintCraft uses.
- **`hits(query)`** (public, also used by `update`): scores all `COMMANDS`, all `ToolGroup`s and all
  tool items, sorts by score with ready entries first, truncates to 12. Returns `Hit
  { group, command, label, detail, icon, ready }`.
- **`hit_message(hit)`** (public): a hit becomes `PalettePick(command)` (command/item) or
  `OpenLeft(Tool(group))` (a group with no command).
- **`palette(app, t)`**: a click-away scrim plus a 560 px card anchored 96 px from the top. The card
  holds a borderless `text_input` (id `PALETTE_INPUT`, autofocused by ⌘K), an "Esc" badge, a divider
  and the scrollable result rows (icon, label, right-aligned detail). **Enter** (`PaletteSubmit`)
  re-runs the top hit; clicking a row runs that row.

### 9.2 Dialogs
**`dialog(app, t)`** (public): a 28 % black scrim with a 560 px centred card — title, × close,
a scrollable body capped at 400 px, and a footer with a Close pill. Three bodies:
- **`properties(app, tab, t)`** (private): a `mode_tab_small` strip for `PropsTab::ALL`
  (Description · Initial View · Security · Fonts) over an alternating-row key/value table. Values
  come from the active document (file name, location, page count, size, signature count), so the
  dialog is live rather than canned.
- **`shortcuts(t)`** (private): 14 shortcut rows in two columns, each a key `badge` plus a
  description — the discoverable version of the keyboard handling in `update`.
- **`about(t)`** (private): name, blurb, version/licence line, and Discord / handbook pills.

### 9.3 Toast
**`toast(app, t)`**: a bottom-centred dark rounded pill with the last message. `App::clock` is
advanced by the 500 ms tick subscription; a toast older than 3.5 s is cleared. This is where every
simulated action reports itself ("Rotate pages (3)", "Copied the document metadata as JSON").

---

## 10. Root composition
**File:** `src/view.rs`

- **`view(app)`**: builds the workspace `row` conditionally — `left` (if `left_open`), `canvas`
  (always, `Fill`), `right` (if `Some`), `rail` (always) — over a column of `tab_strip`, `mode_bar`
  and a divider. Then pushes the floating layers in order: `menu_layer`, `palette`, `dialog`, `toast`.
- **`menu_layer(app, t)`** (private): a transparent, click-away scrim plus the dropdown anchored
  exactly below the chrome (`TAB_STRIP_HEIGHT + MODE_BAR_HEIGHT + 1`), 10 px from the left.

---

## 11. State and wiring
**File:** `src/app.rs` · **PrintCraft:** `lib.rs` (`PrintCraftApp`, `Mode`, `LeftPanel`, …)

### 11.1 Vocabulary
`Mode` (5 variants, with `label()` and `left()`), `LeftPanel::{AllTools, Tool(id)}`,
`RightPanel` (7 variants with `icon()`/`title()`), `Fit::{Page, Width, Height, Actual}`,
`PageLayout::{Continuous, Single, TwoUp}`, `Dialog::{Properties(PropsTab), Shortcuts, About}` and
`PropsTab` (4 variants with `label()`).

### 11.2 Document model
`Doc` (name, path, pages, dirty, size, comments, outline, fields, layers, attachments, signed) with
`display_name()`; `Comment` (author, initials, page, time, body, replies, resolved);
`OutlineItem` (recursive children); `Field`, `Layer`, `Attachment`; `RecentFile`. `App::default()`
seeds a 13-page "PrintCraft Showcase" with 2 threads, a nested 5-node outline, 4 form fields,
3 layers and 2 attachments, plus 3 recent files.

### 11.3 `App` state
Workspace: `theme`, `mode`, `left`/`left_open`, `right`, `active`, `docs`, `recent`.
Surfaces: `palette_open`, `palette_query`, `dialog`, `toast`, `menu`.
Find: `find_open`, `find_query`. View: `page`, `zoom`, `fit`, `layout`, `page_input`.
Organize: `organizing`, `selected_pages`. Comments: `comment_draft`. Timing: `clock`.

Helpers: `tokens()`, `doc()`, `doc_mut()`, `notify()`, `match_count()`, `effective_zoom()`.

### 11.4 Messages and `update`
One flat `Message` enum grouped as Chrome / Palette / Find / View / Right panel / Transient /
Keyboard. `update` is a single `match`; it returns a `Task` where side effects are needed.
Notable behaviours:
- **`run_command(&mut self, command) -> Task<Message>`** is the single dispatch point for the
  palette, the menus and the left panel. It handles real commands (open, new, save, print,
  properties, theme, zoom, fit, layout, find, organize, panel) and, for anything else, *navigates to
  the tool that owns it* or reports honestly that it is a placeholder.
- **`OpenRequested` / `OpenPicked`** — `Task::perform(async { rfd::FileDialog…pick_file() }, …)`, then
  `add_opened_document` reads the name and size, adds a tab, and prepends a recent entry.
- **`file.copy_metadata`** — `metadata_json()` serialises the document model with `serde` +
  `serde_json` and `iced::clipboard::write` puts it on the clipboard.
- **`PaletteSubmit`** re-enters `update` with the top hit, so Enter and click share one path.
- **`Tick`** advances `App::clock` and expires the toast.
- **`Key`** handles Escape (closes palette → dialog → menu → find, in that priority), ⌘K (palette,
  with focus request) and ⌘F (find, with focus request).

### 11.5 `subscription(app)`
`keyboard::listen()` mapped to `Message::Key`, batched with `iced::time::every(500 ms)` for the tick.
This is the only subscription; everything else is event-driven.

### 11.6 `human_size(bytes)`
Formats a byte count as B / KB / MB for the recent-files list and newly opened files.

---

## 12. Keyboard and pointer summary

| Input | Result |
| --- | --- |
| `⌘K` (or the search box) | open the command palette and focus its field |
| `⌘F` | open the find bar and focus it |
| `Esc` | close, in order: palette, dialog, menu, find |
| `Enter` in the palette | run the top result |
| `Enter` in the find bar | next match |
| `Enter` in the page box | jump to that page |
| Click a scrim | close the palette / menu |
| Hover | every icon button, row, tab and menu item has a token-driven hover state and, where it is not obvious, a tooltip |

## 13. What is simulated

This is a design demo, not a PDF engine. The page content, search hit counts and page thumbnails are
generated; Print, Print-to-PDF, OCR, redaction and signature operations report themselves through the
toast instead of running. What *is* real: theme switching, mode/panel navigation, the full catalogue
and command registry, the command palette, find, organize-pages selection and deletion, comment
creation/resolution, layer visibility, page navigation and zoom, the OS file dialog, JSON metadata
export to the clipboard, and every hover/focus/disabled state.
