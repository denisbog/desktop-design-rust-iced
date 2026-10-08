# PrintCraft — iced design demo

A minimal re-creation of [PrintCraft](https://github.com/storytold/printcraft)'s look and feel,
rendered with **[iced](https://iced.rs) 0.14** instead of egui/eframe. It is a running application
that covers every major element of PrintCraft's UI — window chrome, the mode bar and its menus, the
"All tools" catalogue, the pasteboard and its page, the right panel family, the rail, the ⌘K command
palette, dialogs and toasts — built on the same design tokens and the same information architecture
as the original.

PrintCraft ships its design as data and tokens (`crates/ui-egui/src/theme.rs`, `widgets.rs`,
`palette.rs`, and `crates/engine/src/catalog.rs`). This project keeps that split and swaps only the
toolkit: `theme.rs` mirrors `Tokens` field for field, `catalog.rs` mirrors the tool/command tables,
and every UI module is the iced counterpart of an egui module.

## Run it

```sh
cargo run          # debug
cargo run --release
cargo test         # asserts every catalogue icon is embedded
```

Requires a GPU-capable desktop session (iced's default wgpu renderer).

## Try these

| Action | Where |
| --- | --- |
| Switch light ⇄ dark | `moon`/`sun` in the tab strip — pages stay white in both, as PrintCraft's Dark Gray theme does |
| Open a PDF from disk | **Open** in the tab strip, or "Open file" on the Home tab (real `rfd` dialog; name + size become the new tab) |
| Command palette | Click **Find tools and commands**, or press <kbd>⌘K</kbd>; type `organ`, `comment`, `fit`… |
| Open a tool | Any row in the left panel's **All tools**, or a "Recommended tools" card on Home |
| Find in document | <kbd>⌘F</kbd>, or View ▸ Find… in the **Menu** dropdown |
| Organize pages | Left panel ▸ Organize pages ▸ Page grid; select thumbnails, then Rotate/Delete/Done |
| Comments | Right rail ▸ Comments; write one and press **Comment**, then Resolve/Reopen it |
| Bookmarks / Layers / Fields / Attachments | Right rail icons; Layers toggle visibility, Bookmarks jump pages |
| Zoom, fit, page jump | The rail's bottom cluster; type a page number and press Enter |
| JSON metadata | Menu ▸ File ▸ **Copy document metadata as JSON** (serde + serde_json → clipboard) |
| Properties, shortcuts, about | `info` in the mode bar, `circle-help` in the tab strip |

<kbd>Esc</kbd> closes, in priority order: palette → dialog → menu → find.

## Project layout

```
build.rs              generates the embedded icon table from assets/icons
assets/fonts          Inter 400/500/600 + JetBrains Mono (the OFL faces PrintCraft bundles)
assets/icons          the Lucide SVG set (ISC)
src/main.rs           window setup: theme fn, subscription, fonts, default font
src/theme.rs          design tokens, theme derivation, font helpers
src/icons.rs          SVG handles + tinted icon widget (+ catalogue icon test)
src/catalog.rs        tool groups, sections, items, commands, menus  (pure data)
src/widgets.rs        mode_tab, pill/ghost/icon buttons, search_box, menu_item, badge, tip…
src/app.rs            App state, document model, Message enum, update, subscription
src/view.rs           root composition + floating layers
src/ui/chrome.rs      tab strip, mode bar, menu dropdown
src/ui/left.rs        All tools, tool detail, item rows
src/ui/rail.rs        panel toggles + page/zoom cluster
src/ui/right.rs       Comments, Bookmarks, Pages, Fields, Layers, Attachments, Signatures
src/ui/canvas.rs      Home/Reader/Organize switch, page and thumbnail rendering, find bar
src/ui/home.rs        welcome, community card, recommended tools, recent files
src/ui/overlays.rs    command palette, dialogs, toast
COMPONENTS.md         component-by-component reference (every widget, described)
```

## Dependencies

Same stack as PrintCraft, minus egui and the PDF engine:

| Crate | Why |
| --- | --- |
| `iced` (0.14, `svg` + `smol`) | the toolkit that replaces `egui`+`eframe`; `svg` draws the icon set, `smol` provides the timer used by the toast |
| `serde` + `serde_json` | the document model and the "Copy metadata as JSON" command |
| `log` + `env_logger` | the same logging setup the app uses |
| `rfd` | native open-file dialog (PrintCraft's `rfd` dependency) |

PrintCraft's `printcraft-*` crates (engine, render, fonts, …) are replaced by the demo's own
`catalog`/`app`/`ui` modules, since this app renders a simulated page rather than parsing PDFs.

## Design language, in brief

- Neutral chrome, white panels, **one** blue accent (`#1B63E0` light, `#4B8BF5` dark).
- A three-level text hierarchy (`text` / `text_muted` / `text_faint`), all ≥ 4.5:1 against the
  surfaces they sit on.
- 6 px radius for controls, 8–12 px for cards and popups; hairline 1 px dividers instead of heavy
  borders.
- Inter for everything, JetBrains Mono for numerals; icons are never Unicode glyphs, they are Lucide
  SVGs tinted per use.
- Tool families are colour-coded by `hue` (red, purple, blue, green, orange, teal, pink), exactly as
  Acrobat and PrintCraft colour-code them.
- Availability is honest: planned tools stay visible but disabled, with their milestone on the badge.

## Attribution

The Inter and JetBrains Mono fonts (OFL) and the Lucide icons (ISC) are the same assets PrintCraft
bundles; see `assets/fonts/OFL-*.txt` and `assets/icons/LICENSE-lucide.txt` upstream. PrintCraft
itself is MIT OR Apache-2.0.

See [COMPONENTS.md](COMPONENTS.md) for the full component reference.
