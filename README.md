# RetroVerse

**by Navine**

RetroVerse is a base project for a small launcher and its server: a window, a saved list of builds,
an HTTP server that starts and answers a health check, and the shape of the code around all of it.
It is only a starting point. Detecting builds, scanning folders, starting programs, accounts,
profiles, chat and the game-facing endpoints are not implemented, so building this repository gives
a window and a server that do nothing useful on their own. Expect to add a lot before either one is
a real launcher or a real server.

## Layout

### Launcher (`src/`)

- `catalog.rs`, `detect.rs`, `family.rs`, `scan.rs`, `launch.rs`: empty stubs with the intended shape.
- `library.rs`: the saved list.
- `theme.rs` and `main.rs`: the window.

### Server (`server/`)

- `db.rs`: opens a SQLite database and creates the accounts table. This part works.
- `api.rs`, `main.rs`: an HTTP server that starts, binds a port and answers `/health`. This part
  works too.
- `auth.rs`, `mcp.rs`, `og.rs`, `xmpp.rs`, `moderation.rs`: empty stubs with the intended shape —
  sign-in, profiles, the game-facing endpoints, chat/presence, and bans/reports.

## Building

```text
cargo build
cargo build --manifest-path server/Cargo.toml
```

## Running the server

```text
cargo run --manifest-path server/Cargo.toml
```

Starts on `http://127.0.0.1:8080` and answers `GET /health`. Every other route returns the base
project's "not implemented" shape.

## License

MIT, see `LICENSE`.
