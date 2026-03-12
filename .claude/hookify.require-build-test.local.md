---
name: require-build-test
enabled: true
event: stop
action: warn
conditions:
  - field: transcript
    operator: not_contains
    pattern: cargo test
---

**Reminder: tests were not run in this session.**

Before finishing, consider running:
```
cargo build && cargo test
```

This ensures changes compile cleanly and all tests still pass.
