# Blockary

A CLI tool for [time blocking](https://todoist.com/productivity-methods/time-blocking) — a productivity technique where you schedule dedicated blocks of time for specific tasks. Blockary processes Markdown day plans compatible with [Obsidian's Day Planner plugin](https://github.com/ivan-lednev/obsidian-day-planner), syncing them across multiple sources and analyzing tags for time tracking reports.

## Features

- **Sync** day plans across multiple directories (e.g. work vault, personal vault)
- **Pull** calendar events from iCalendar feeds into your day plan
- **Analyze** time spent per tag with hierarchical breakdowns

## Installation

```sh
cargo install --path .
```

## Configuration

Blockary reads `~/.config/blockary.toml`:

```toml
[dirs]

[dirs.work]
path = "/path/to/work/obsidian/vault/daily-notes"
name = "Work"

[dirs.personal]
path = "/path/to/personal/obsidian/vault/daily-notes"
name = "Personal"

[cals]

[cals.work]
uri = "https://calendar.example.com/feed.ics"
```

- **`[dirs]`** — One or more day plan directories (required). Each key becomes an *origin* label.
- **`[cals]`** — iCalendar feeds to pull events from (optional).

## Day Plan Format

Day plan files are Markdown files named with a date (e.g. `2025-03-12.md`). Blockary reads and writes a `## Time Blocks` section:

```markdown
## Time Blocks

- 09:00 - 10:00 Deep work on @project/alpha
- 10:00 - 10:30 (Personal) Morning walk @break
- 11:00 - 12:00 Team meeting @meetings
```

- **Period** (`HH:MM - HH:MM`) — optional; defaults to 30 minutes if omitted
- **Origin** (`(Name)`) — optional label for blocks from another source
- **Tags** (`@tag` or `@parent/child`) — hierarchical tags for time analysis

## Commands

### `blockary sync`

Merges time blocks across all configured directories. Each file gets the full picture — its own blocks plus blocks from other origins, labeled with their source.

```sh
blockary sync
```

Optionally supply an ICS file directly:

```sh
blockary sync --ics-file events.ics
```

### `blockary spent [PERIOD]`

Shows a breakdown of time spent per tag for the given period.

```sh
blockary spent           # today (default)
blockary spent this-week
blockary spent this-month
blockary spent this-year
blockary spent last-week
```

Blocks tagged `@break` are excluded from totals.

### `blockary pull`

Fetches events from configured calendar feeds and inserts them into a day plan file, skipping any that conflict with existing blocks.

```sh
blockary pull                        # today, target dir inferred if only one
blockary pull --date 2025-03-15
blockary pull --date 2025-03-15 --target work
```
