# Configuration

rapi stores its settings in a TOML configuration file at a platform-appropriate location:

| Platform | Path |
|----------|------|
| Linux | `~/.config/rapi/config.toml` |
| macOS | `~/Library/Application Support/rapi/config.toml` |
| Windows | `C:\Users\<user>\AppData\Roaming\rapi\config.toml` |

The directory and file are created automatically on first settings change.

## Format

```toml
[appearance]
theme = "GruvboxDark"
```

## Fields

| Section | Key | Type | Default | Description |
|---------|-----|------|---------|-------------|
| `appearance` | `theme` | String | `"GruvboxDark"` | Name of the iced theme. Must match one of the built-in theme names. |

## Behavior

- **Loading** — The config is read at application startup. If the file is missing or invalid, defaults are used.
- **Saving** — The config is written every time a setting changes (e.g., selecting a new theme). The config directory is created if it doesn't exist.
- **Manual Editing** — The file is human-readable TOML and can be edited manually. Changes take effect on the next app launch.
