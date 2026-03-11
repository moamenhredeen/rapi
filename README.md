# rapi — REST API Testing Tool

A native desktop REST API testing tool built with **Rust** and **Iced**, inspired by Postman.

## Features

- 📦 **Collections** — Send HTTP requests (GET, POST, PUT, DELETE) with params, headers, and body
- 🌍 **Environments** — Manage multiple environments with key-value variables
- ⚙ **Settings** — 22 built-in themes with persistent configuration
- 🎨 **Composable UI** — Built from reusable widget components following the Elm architecture

## Building

```sh
cargo build --release
```

## Running

```sh
cargo run
```

## Configuration

Settings are stored in `~/.config/rapi/config.toml` (Linux), `~/Library/Application Support/rapi/config.toml` (macOS), or `%APPDATA%\rapi\config.toml` (Windows).

## Documentation

The full documentation is built with [mdbook](https://rust-lang.github.io/mdBook/):

```sh
cd docs && mdbook serve
```

## TODO

- [ ] Query parameter substitution from environment variables
- [ ] Authentication (Bearer, Basic, API Key)
- [ ] Request collections with persistence (HTTP file format)
- [ ] Import/export Postman collections
- [ ] Export to OpenAPI format
- [ ] Scripting support
