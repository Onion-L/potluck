# ptlk - Potluck TUI

A fast, interactive terminal user interface for reading Potluck news.

## Installation

```bash
cargo build --release
cp target/release/ptlk ~/.local/bin/
```

## Usage

```bash
# Default (localhost:3000)
ptlk

# Custom API URL
ptlk --api-url https://potluck.example.com

# With more articles
ptlk --limit 100
```

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` | Go to top |
| `G` | Go to bottom |
| `Ctrl+d` / `PageDown` | Page down |
| `Ctrl+u` / `PageUp` | Page up |
| `Enter` | Expand / Open in browser |
| `Space` | Toggle expand |
| `o` | Open article in browser |
| `x` | Collapse all |
| `r` | Refresh |
| `q` / `Esc` | Quit |

## Environment Variables

- `POTLUCK_API_URL` - API base URL (default: `http://localhost:3000`)
