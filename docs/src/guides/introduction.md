# Introduction

**rapi** is a REST API testing tool built with Rust and [Iced](https://iced.rs). It provides a Postman-style interface for sending HTTP requests, managing environments with variables, and inspecting responses — all from a native desktop application.

## Features

- **HTTP Client** — Send GET, POST, PUT, and DELETE requests with custom headers, query parameters, and request bodies.
- **Collections Screen** — Main workspace with a URL bar, request configuration tabs (Params, Headers, Body), and a response viewer with JSON syntax highlighting.
- **Environments Screen** — Create and manage multiple environments, each with its own set of key-value variables.
- **Settings Screen** — Choose from 22 built-in themes (Gruvbox, Tokyo Night, Catppuccin, Dracula, Nord, and more).
- **Persistent Configuration** — Settings are saved to a TOML config file and restored on startup.
- **Composable Architecture** — The UI is built from small, reusable widget components following the Elm architecture pattern.

## Tech Stack

| Component | Crate |
|-----------|-------|
| GUI Framework | `iced` 0.13 |
| HTTP Client | `reqwest` 0.12 |
| Async Runtime | `tokio` |
| Serialization | `serde` + `toml` |
| Config Paths | `dirs` |

