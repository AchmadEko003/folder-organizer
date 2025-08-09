# Folder Organizer Tool

A simple CLI tool written in Rust to organize files in a folder by their extension.

## Features
- Automatically sorts files into subfolders based on their file extension (e.g., `.png` files go to a `png` folder).
- Easy to use from the command line.
- Useful for cleaning up messy directories.

## Usage
```
cargo run
```

## Example
Suppose your folder not empty and contains:
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
