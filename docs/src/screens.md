# Screens

rapi has three main screens, accessed via the activity bar on the left:

| Icon | Screen | Description |
|------|--------|-------------|
| 📦 | Collections | Send and test HTTP requests |
| 🌍 | Environments | Manage environments and variables |
| ⚙ | Settings | App preferences (theme, etc.) |

The `AppScreen` is the root that holds the current route, all screen states, and the app configuration. It renders the activity bar alongside the active screen's content and a status bar at the bottom.

## Layout

```
┌──────┬─────────────────────────────────┐
│      │                                 │
│  📦  │     Active Screen Content       │
│  🌍  │                                 │
│  ⚙   │                                 │
│      │                                 │
├──────┴─────────────────────────────────┤
│ Status Bar                             │
└────────────────────────────────────────┘
```
