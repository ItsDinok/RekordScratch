# RekordScratch
> A Rust-based tool to recover MP3 files from Rekordbox-playable USB files: reverting them to playlist or genre layout.

## What is RekordScratch?

**RekordScratch** lets you recover MP3s from a Rekordbox USB — even if the original Rekordbox database is gone.  
Whether you lost your library, switched machines, or just found a USB on the floor (I won’t ask), this tool helps you extract tracks and rebuild their original playlist or genre structure.

## Features

- Extracts MP3s from Rekordbox-exported USBs
- Rebuilds the original folder structure based on track metadata, or title if metadata isn't available
- It automatically ignores non-rekordbox USB devices
- Simple UI structure
- Does **NOT** parse proprietary Rekordbox pdb structure (I enjoy not being in lawsuits)

## Usage

1. Plug in your Rekordbox-formatted USB.
2. Export playlist.txt files from Rekordbox, store them somewhere in a 'Playlists' folder
3. Place the 'Playlists' folder next to the executable
4. Run the tool 

Build with: 
```bash
cargo run --release
```

4.5. Or use the latest release

If you have a custom playlusts folder you can also use
```bash
cargo run --release -t "C:/Users/path/to/Playlists"
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
2. Less rigidity about the Playlists folder
