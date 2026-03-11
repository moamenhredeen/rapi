# Environments Screen

The Environments screen lets you create and manage multiple environments, each with its own set of key-value variables.

## Layout

```
┌─────────────────────────────────────────────┐
│ Environments                                │
│                                             │
│ [Default ▾] [New environment...] [+] [🗑]   │
│ ─────────────────────────────────────────── │
│ Variables for: Default                      │
│ Key          Value                          │
│ [base_url ] [https://api.example.com     ] ✕│
│ [api_key  ] [sk-1234...                  ] ✕│
│ [+ Add]                                     │
└─────────────────────────────────────────────┘
```

## Usage

- **Select** an environment from the dropdown to view and edit its variables.
- **Create** a new environment by typing a name and clicking **+** or pressing Enter.
- **Delete** the current environment with the 🗑 button (at least one environment must remain).
- **Add/remove variables** using the key-value editor within the selected environment.

Each environment holds an independent set of variables that can be used in future features like variable substitution in URLs, headers, and request bodies.
