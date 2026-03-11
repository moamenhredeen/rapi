# Composable Widgets

Every widget in rapi follows the same pattern: a struct holding state, a `Message` enum, an `update()` method, and a `view()` method. This mirrors the Elm architecture at the component level.

## Widget Pattern

```rust
pub struct MyWidget {
    // widget state
}

#[derive(Debug, Clone)]
pub enum Message {
    // widget-specific messages
}

impl MyWidget {
    pub fn update(&mut self, message: Message) {
        // handle state changes
    }

    pub fn view(&self) -> Element<'_, Message> {
        // return iced UI elements
    }
}
```

## Composing in a Screen

Screens own widget instances and map their messages:

```rust
pub struct MyScreen {
    url_bar: UrlBar,
}

pub enum Message {
    UrlBar(url_bar::Message),  // wrap child message
}

// In update():
Message::UrlBar(msg) => self.url_bar.update(msg),

// In view():
self.url_bar.view().map(Message::UrlBar)  // map child -> parent message
```

## Available Widgets

| Widget | Description |
|--------|-------------|
| `ActivityBar` | Narrow icon sidebar for screen navigation. Takes a list of items with icons and routes. |
| `UrlBar` | HTTP method picker, URL text input, and send button. Supports Enter-to-send. |
| `TabBar` | Generic tab strip. Works with any `T: Clone + PartialEq`. Highlights the active tab. |
| `KeyValueEditor` | Editable list of key-value pairs with add/remove. Used for headers, params, and environment variables. |
| `BodyEditor` | Text editor with JSON syntax highlighting for request bodies. |
| `ResponseViewer` | Read-only response display with status code badge (color-coded), timing, and body size. |
| `StatusBar` | Bottom bar showing application status messages. |
