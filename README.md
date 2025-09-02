# Folder Organizer Tool

A small CLI utility, written in Rust, that moves files inside a folder into subfolders based on their extension.

## Features

- Sorts files into type folders: `images/<ext>`, `documents/<ext>`, `videos/<ext>`, and `others/<ext>`.
- Accepts a folder path as a command-line argument or interactively (run then paste/drag‑and‑drop the path and press Enter).
- Recognizes many common image, document, and video extensions (see `src/utils.rs`).

## Requirements

- Rust (stable) — install from https://rust-lang.org
- Tested on Linux (should work on macOS/Windows with minor differences in path/permissions).

## Build

Build a debug binary:

```bash
cargo build
```

Build an optimized release binary:

```bash
cargo build --release
```

Binary will appear at `target/debug/folder-organizer` or `target/release/folder-organizer`.

## Usage

- As a single-shot command (preferred when automating):

```bash
# with the built binary
./target/release/folder-organizer "/path/to/folder"

# or with cargo (development)
cargo run -- "/path/to/folder"
```

- Interactive / drag‑and‑drop mode:

```bash
# run without arguments, then paste or drag a folder into the terminal and press Enter
./target/debug/folder-organizer
```

The program trims surrounding quotes from pasted paths, so dragging a folder from the file manager into the terminal usually works.

## What it does

For each file in the provided folder (non-recursive), the tool:

- Detects the file extension (lowercased).
- Classifies it as image, document, video or other using `src/utils.rs`.
- Creates the destination folder (`images/<ext>`, `documents/<ext>`, `videos/<ext>`, or `others/<ext>`) and moves the file there.

Example before:

```
photo.png
report.pdf
notes.txt
movie.mp4
```

After running:

```
images/png/photo.png
documents/pdf/report.pdf
documents/txt/notes.txt
videos/mp4/movie.mp4
```

## Supported types

The exact lists of extensions the program treats as images/documents/videos are in `src/utils.rs`. You can edit that file to add or remove extensions.

## Safety & notes

- The tool moves files (uses rename). If you want a safe copy-first behavior, modify `organize_folder` to copy instead of rename.
- Running on system folders may require permissions; run with an account that has rights to move files in the target folder.
- The current implementation is non-recursive: it only organizes files in the top level of the provided folder.
- Be cautious running against large system directories; test on a small sample first.

## Contributing

Contributions welcome. Edit `src/utils.rs` to tweak extension lists or improve classification. Open a PR with your changes.
