# Project Structure

```
src/
├── main.rs                          # Entry point, app configuration
├── config/
│   └── mod.rs                       # TOML config loading/saving
├── http/
│   ├── mod.rs
│   ├── method.rs                    # HttpMethod enum (Get, Post, Put, Delete)
│   ├── request.rs                   # Request model (url, method, headers, params, body)
│   ├── response.rs                  # Response model (status_code, headers, body, duration)
│   └── client.rs                    # Async HTTP execution via reqwest
├── screens/
│   ├── mod.rs
│   ├── route.rs                     # Route enum (Collections, Environments, Settings)
│   ├── app_screen.rs                # Root screen — activity bar, routing, config
│   ├── home_screen.rs               # Collections screen — composes request/response widgets
│   ├── environments_screen.rs       # Environment & variable management
│   └── settings_screen.rs           # Theme picker and app preferences
└── widgets/
    ├── mod.rs
    ├── activity_bar.rs              # Narrow icon navigation bar
    ├── url_bar.rs                   # HTTP method picker + URL input + send button
    ├── tab_bar.rs                   # Generic reusable tab strip
    ├── key_value_editor.rs          # Editable key-value pair list
    ├── body_editor.rs               # Request body text editor with highlighting
    ├── response_viewer.rs           # Response display with status badge
    ├── side_bar.rs                  # Sidebar item/header helpers
    └── status_bar.rs                # Bottom status bar
```

## Module Responsibilities

| Module | Purpose |
|--------|---------|
| `config` | Load/save `~/.config/rapi/config.toml`, map theme names to iced types |
| `http` | Request/response models, async HTTP client, method enum |
| `screens` | Page-level state and composition of widgets |
| `widgets` | Reusable, self-contained UI components |
