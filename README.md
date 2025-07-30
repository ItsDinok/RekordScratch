# RekordScratch

A terminal-based audio annotation and scratch tool written in Rust.

## 🎧 What is RekordScratch?

RekordScratch is a command-line tool designed to help DJs, researchers, and developers **scrub through audio**, **mark significant timestamps**, and **create structured logs** of audio files using keyboard input. It's ideal for use cases like:

- Beat matching and tempo analysis
- Track annotation
- Machine learning preprocessing
- Custom DJ workflow tools

## 🚀 Features

- Terminal-based UI using `crossterm`
- Real-time keypress interaction
- Timestamped event logging
- Customizable key bindings (soon)
- Output in a structured log format

## 🛠️ Installation

Ensure you have Rust installed. Then clone and build:

```bash
git clone https://github.com/ItsDinok/RekordScratch.git
cd RekordScratch
cargo build --release
