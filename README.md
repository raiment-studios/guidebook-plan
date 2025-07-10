# 🍁 guidebook-plan

A command-line utility to help keep on track with your daily routine.

## Status

Early prototype. It's **functional but a little rough around the edges**. The code needs a fair amount of clean-up and the user experience could use some attention.

## Quick start

TODO: still working on this!

**Requirements**: [Rust](https://rustup.rs/) must be installed to use `cargo`

```bash
# Installation
cargo install --git https://github.com/raiment-studios/guidebook-plan

# Initilization (run once per computer)
guidebook-plan init

# Show the current plan
guidebook-plan

# Edit the plan
#
# TODO: editing the plan currently requires manual modification of the
# plan YAML file. The format is not yet documented beyond what's in the
# source code! That's why this is still a prototype!!
guidebook-plan open
```

## Development

### Roadmap

#### v1.0

-   [ ] Allow different routines on weekends
-   [ ] Improved edit/update workflow

#### v0.2-prototype

-   [x] Convert TypeScript prototype to Rust
-   [x] GitHub login on `init`
-   [ ] Remove hard-coded VS Code reference in `open` command
-   [ ] One-liner installation for users
-   [ ] Provide documentation on the plan format
-   [ ] Add helpful screenshots to the README
-   [ ] Design clean-up on the extravagent color usage
-   [ ] Rename `open` -> `edit`

#### v0.1-prototype

-   [x] Minimal TypeScript prototype to test usefulness
-   [x] Sub-command to clone remote repo

#### Backlog (aka someday)

-   [ ] Mobile & web support

### History

The original prototype for `guidebook-plan` was written in TypeScript around July 2025. Once the prototype proved to be sufficiently useful, it was ported to Rust using Claude Sonnet 4 to automate the initial conversion, which was then manually edited to sanitize the code.
