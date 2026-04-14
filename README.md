# ImgForge

A desktop image format converter built with Rust and iced. Load an image, pick a target format, and export — no bloat, no internet connection required.

---

## Screenshots

<img width="2940" height="1904" alt="image" src="https://github.com/user-attachments/assets/59fb6d5e-03dc-47c7-b984-eadbea5d895d" />
<img width="2940" height="1912" alt="image" src="https://github.com/user-attachments/assets/15beb9aa-5a40-4d80-ae16-30c881c060d9" />

---

## Features

- Supports JPG, PNG, TIFF, BMP and WebP
- Live image preview with pan and zoom
- Automatically excludes the current format from the export list
- Shows image dimensions and format in the status bar
- Cross-platform: Linux, Windows and macOS

---

## Built With

| Crate | Purpose |
|-------|---------|
| [iced](https://github.com/iced-rs/iced) | GUI framework |
| [iced_aw](https://github.com/iced-rs/iced_aw) | Additional widgets (SelectionList) |
| [image](https://github.com/image-rs/image) | Image decoding and encoding |
| [rfd](https://github.com/PolyMeilex/rfd) | Native file dialogs |
| [chrono](https://github.com/chronotope/chrono) | Date and time formatting |

---

## Supported Formats

| Format | Open | Export |
|--------|------|--------|
| JPG / JPEG | yes | yes |
| PNG | yes | yes |
| TIFF | yes | yes |
| BMP | yes | yes |
| WebP | yes | yes |

---

## Prerequisites

- Rust toolchain — https://rustup.rs

---

## Installation

### Build from source
```bash
git clone https://github.com/swastikgn/imgforge
cd imgforge
cargo build --release
```

The binary will be at `target/release/imgforge`.

### Run
```bash
cargo run --release
```

---

## Usage

1. Launch the app
2. Click **Select an Image** to open a file
3. The current format is shown in the status bar at the bottom
4. Pick a target format from the list on the right
5. Click **Export** and choose where to save the file
6. Use **Clear** to reset and load a different image

---

## Known Limitations

- Animated GIFs are not supported — iced's image viewer renders static frames only
---
