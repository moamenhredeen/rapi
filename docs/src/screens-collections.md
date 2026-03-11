# Collections Screen

The Collections screen is the main workspace for building and sending HTTP requests. It is composed of several widgets:

## Layout

```
┌─────────────────────────────────────────────┐
│ [GET ▾] [Enter URL...              ] [Send] │
├──────────────────────┬──────────────────────┤
│ Request              │ Response             │
│ [Params|Headers|Body]│ 200  45ms  1234 bytes│
│                      │                      │
│ Key: ___  Value: ___ │ {                    │
│ Key: ___  Value: ___ │   "data": "..."      │
│ [+ Add]              │ }                    │
└──────────────────────┴──────────────────────┘
```

## Components

- **URL Bar** — Pick the HTTP method (GET, POST, PUT, DELETE), enter the URL, and press Send or hit Enter.
- **Request Tabs** — Switch between Params, Headers, and Body tabs:
  - **Params** — Key-value editor for query parameters.
  - **Headers** — Key-value editor for custom HTTP headers.
  - **Body** — Text editor with JSON syntax highlighting for the request body.
- **Response Viewer** — Displays the response with:
  - Color-coded status badge (green for 2xx, yellow for 3xx, red for 4xx/5xx)
  - Response time in milliseconds
  - Body size in bytes
  - JSON syntax highlighting

## Sending a Request

1. Select the HTTP method from the dropdown
2. Enter the target URL
3. (Optional) Add query parameters, headers, or a request body
4. Click **Send** or press **Enter** in the URL field
5. The response appears in the right panel with status, timing, and highlighted body
