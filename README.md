# RetroVerse

**by Navine**

RetroVerse is a base project for a small launcher: a window, a saved list of builds and the shape of
the code around them. It is only a starting point. Detecting builds, scanning folders and starting
programs are not implemented, so building this repository gives a window that does nothing useful.

## Layout

- `src/catalog.rs`, `detect.rs`, `family.rs`, `scan.rs`, `launch.rs`: empty stubs with the intended shape.
- `src/library.rs`: the saved list.
- `src/theme.rs` and `src/main.rs`: the window.

## Building

```text
cargo build
```

## License

MIT, see `LICENSE`.
