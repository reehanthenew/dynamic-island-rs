# Dynamic Island RS

A small Rust project that recreates the feel of a Dynamic Island-style notification bar using GTK4.

## Features

- minimal / compact / expanded display modes
- animated CSS-driven transitions
- modular project structure for plugins and themes
- ready to evolve toward a more dynisland-like plugin architecture

## Run

```bash
cargo run --release
```

## Notes

This project is intentionally lightweight and is designed as a clean foundation for expansion into:

- activity providers
- layout managers
- plugin modules
- theming and custom widgets

## Dependencies

- GTK 4
- Rust toolchain

On Debian/Ubuntu you may need:

```bash
sudo apt install libgtk-4-dev pkg-config
```
