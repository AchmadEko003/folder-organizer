# Folder Organizer Tool

A simple CLI tool written in Rust to organize files in a folder by their extension.

## Features
- Automatically sorts files into subfolders based on their file extension (e.g., `.png` files go to a `png` folder).
- Easy to use from the command line.
- Useful for cleaning up messy directories.

## Usage
```
cargo run -- "C:\path\to\your\folder"
```

## Example
Suppose your folder contains:
```
photo.png
report.pdf
notes.txt
```
After running the tool, your folder will look like:
```
png/photo.png
pdf/report.pdf
txt/notes.txt
```

## Requirements
- Rust (https://rust-lang.org)
- Windows, Linux

## Customization
You can modify the code to group certain extensions into a single folder by editing the logic in `organize_folder` in `src/main.rs`.
