---
name: no-push-to-main
enabled: true
event: pre_tool_use
action: warn
conditions:
  - field: tool_name
    operator: equals
    pattern: Bash
  - field: tool_input.command
    operator: matches
    pattern: "git push.*\\bmain\\b|git push.*\\bmaster\\b"
---

**Warning: you are about to push directly to `main`/`master` on the remote.**

This is usually wrong. If the goal is to open a PR, create a feature branch instead:
```
git checkout -b <branch-name>
git push <remote> <branch-name>
gh pr create ...
```

Only proceed with a direct push to `main` if the user has explicitly confirmed this is their intent.
