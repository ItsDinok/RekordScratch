# RekordScratch

A Rust-based tool to recover MP3 files from Rekordbox-playable USB files: reverting them to playlist or genre layout.

## What is RekordScratch?

**RekordScratch** allows you to copy playlists off of a rekordbox USB in the event that the tracks are no longer on your master rekordbox database, or if you found a USB on the floor and inexplicably plugged it in, finding out it was a rekordbox USB. Rekordbox presently provides **no native way** to export this data back to your host machine, providing a frustrating experience involving manually ccopying and reassembling the playlists yourself.

Whether you’ve lost your source library or just want your organization back, this tool extracts MP3s from a Rekordbox-playable USB and restores them to their original hierarchy — if recoverable.

## Features

- Extracts MP3s from Rekordbox-exported USBs
- Rebuilds the original folder structure based on track metadata and file paths
- Currently reads the USB's `PIONEER` structure directly (no RB database parsing yet)
- It automatically ignores non-rekordbox USB devices

## Usage

1. Plug in your Rekordbox-formatted USB.
2. Export playlist.txt files from Rekordbox, store them somewhere in a 'Playlists' folder
2. Run the tool from the command line in the parent directory of the 'Playlists' folder:

```bash
cargo run --release
```

## Requirements

Currently the `export.pdb` file native to rekordbox USB sticks cannot be read, and even if it could, that would likely cross a legal boundary (I do not want AlphaTheta on my back), as such, some legwork needs to be done.

1. A folder needs to be placed in the same directory as the executable containing playlist.txt files for each playlist you want to export
2. It should be run as administrator
3. Custom paths can be provided for the playlists folder using `-t C:/Users/path/to/playlists`

4. Your MP3 files need to be sourced legally. This isn't just ethical advice, legally sourced MP3s have metadata that this tool relies on extensively. If it isn't high quality it won't work.

If any help is needed, `-h` or `--help` both bring up a currently small help menu.

## Intended upgrades

1. Support for .wav files
2. Better UI (using ratatui)
3. Algorithmic efficiency
4. Less rigidity about the Playlists folder
