# RekordScratch

🧩 A Rust-based tool to **recover MP3 files** from Rekordbox-formatted USB drives, restoring them to their original folder layout.

## 🎛️ What is RekordScratch?

**RekordScratch** solves a simple but frustrating problem for DJs:  
**Once you've exported tracks to a USB using Rekordbox, there's no built-in way to get them back in their original folder structure.** RekordScratch fixes that.

Whether you’ve lost your source library or just want your organization back, this tool extracts MP3s from a Rekordbox-playable USB and restores them to their original hierarchy — if recoverable.

## 🔧 Features

- ✅ Extracts MP3s from Rekordbox-exported USBs
- ✅ Rebuilds the original folder structure based on track metadata and file paths
- ⚠️ Currently reads the USB's `PIONEER` structure directly (no RB database parsing yet)

## ⚙️ Usage

1. Plug in your Rekordbox-formatted USB.
2. Run the tool from the command line:

```bash
cargo run --release
