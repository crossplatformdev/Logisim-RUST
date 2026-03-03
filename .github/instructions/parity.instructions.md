---
applyTo: "docs/**"
---

# Parity Instructions

## Purpose

`docs/PARITY_MATRIX.md` is the single source of truth for implementation status against
Logisim-Evolution v4.1.0. It must be kept accurate and up-to-date.

## Status definitions

| Status | Meaning |
|--------|---------|
| `implemented` | Behavior matches upstream in normal usage |
| `tested` | At least one automated test exercises the behavior |
| `asset-complete` | Required upstream assets (icons, resources, etc.) are present |
| `compatible` | File format / wire protocol is interoperable with upstream |
| `partial` | Core behavior present but gaps remain (describe them) |
| `missing` | Not yet implemented |

## Update rules

1. After implementing any feature, update the corresponding row in `docs/PARITY_MATRIX.md`.
2. Never mark a row `implemented` or `tested` unless the code and test actually exist.
3. Never remove a row. If a subsystem is out of scope, mark it `missing` with a note.
4. The **Summary** table at the bottom must be recalculated after every edit.

## New subsystem additions

When upstream v4.1.0 has a subsystem not yet tracked in the matrix, add it with status `missing`
before implementing, then update to `partial` / `implemented` / `tested` as work progresses.

## Checklist before committing docs changes

- [ ] Every newly implemented feature has an updated row.
- [ ] Summary counts match the detailed rows.
- [ ] No row claims `implemented` or `tested` without matching code/tests.
