# Message Routing

rapi uses a hierarchical message system where each level of the UI defines its own `Message` enum. Messages flow upward through mapping.

## Hierarchy

```
AppScreen::Message
├── Navigate(Route)
├── Collections(home_screen::Message)
│   ├── UrlBar(url_bar::Message)
│   ├── Params(key_value_editor::Message)
│   ├── Headers(key_value_editor::Message)
│   ├── Body(body_editor::Message)
│   ├── Response(response_viewer::Message)
│   ├── SelectRequestTab(RequestTab)
│   └── RequestDone(Result<Response, String>)
├── Environments(environments_screen::Message)
│   ├── SelectEnvironment(String)
│   ├── Variables(key_value_editor::Message)
│   ├── UpdateNewEnvName(String)
│   ├── AddEnvironment
│   └── RemoveEnvironment
└── Settings(settings_screen::Message)
    └── ThemeSelected(Theme)
```

## How It Works

1. A widget emits its own message (e.g., `url_bar::Message::Send`)
2. The parent screen receives it wrapped (e.g., `Message::UrlBar(url_bar::Message::Send)`)
3. The screen's `update()` unwraps and delegates, or handles it directly
4. The app screen receives screen messages wrapped (e.g., `Message::Collections(home_screen::Message::...)`)

This keeps each component decoupled — widgets don't know about screens, and screens don't know about the app root.
