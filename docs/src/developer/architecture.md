# Architecture

rapi follows the **Elm architecture** (Model–Update–View) which is the idiomatic pattern for Iced applications. The codebase is organized into three main layers:

1. **Screens** — Top-level pages (Collections, Environments, Settings) that compose widgets and manage page-level state.
2. **Widgets** — Small, reusable UI components (UrlBar, TabBar, KeyValueEditor, etc.) each with their own state, messages, `update()`, and `view()`.
3. **HTTP Module** — Separated network concerns with typed request/response models and an async client.

## Design Principles

- **Composability** — Each widget is self-contained. Screens compose widgets and map their messages upward.
- **Message Mapping** — Child widgets define their own `Message` enum. Parent screens wrap them (e.g., `Message::UrlBar(url_bar::Message)`) and use `.map()` to convert between message types.
- **Separation of Concerns** — HTTP logic never lives in UI code. The `http` module owns all network operations.
- **Theme-Aware Styling** — All colors come from `theme.extended_palette()`, never hardcoded values. Changing the theme updates the entire UI.
