# 🍁 guidebook-plan

A command-line utility to help keep on track with your daily routine.

Designed for people who like time-boxing, daily routines, and work in the terminal on daily basis.

## Status

Early prototype. It's **functional but a little rough around the edges**. The code needs a fair amount of clean-up and the user experience could use some attention.

## Installation

**Requires**: [Rust](https://rustup.rs/) to be installed for installation.

```bash
cargo install --git https://github.com/raiment-studios/guidebook-plan
```

## Usage

**Initialization**: on your first use, run `guidebook-plan init` to initialize the local and/or remote data directory. This only has to be run once per computer.

```bash
guidebook-plan init
```

**View the routine**: the main command is `guidebook-plan show` which shows where you should be in your daily routine based on the current day and time. This is the default command so running `guidebook-plan` alone will suffice (note if you use this a lot, creating a local `plan` shell alias may make sense!):

```bash
guidebook-plan
```

**Edit the routine**: HEADS UP! This is why this is still a prototype! There's an `guidebook-plan open` command which is hardcoded to open the underlying routine file in [Visual Studio Code](https://code.visualstudio.com/). The plan file format is not complex but it also is not yet documented beyond looking at the source code.

```bash
guidebook-plan open
```

## Development

### Contributing

I'd be shocked if anyone other than me looks at this repo! Let's start from there!

### Roadmap

#### v1.0

-   [ ] Allow different routines on weekends
-   [ ] Improved edit/update workflow
-   [ ] Add versioning to plan format
-   [ ] Import initial routine from GitHub repo templates
-   [ ] Bidirectional code sync on the monorepo publish script

#### v0.2-prototype

-   [x] Convert TypeScript prototype to Rust
-   [x] GitHub login on `init`
-   [ ] Remove hard-coded VS Code reference in `open` command
-   [x] One-liner installation for users
-   [ ] Provide initial documentation on the plan format
-   [ ] Add helpful screenshots to the README
-   [ ] Design clean-up on the extravagent color usage
-   [ ] Rename `open` -> `edit`

#### v0.1-prototype

-   [x] Minimal TypeScript prototype to test usefulness
-   [x] Sub-command to clone remote repo

#### Backlog (aka maybe-someday features)

-   [ ] Mobile & web support

### History

The original prototype for `guidebook-plan` was written in TypeScript around July 2025. Once the prototype proved to be sufficiently useful, it was ported to Rust using Claude Sonnet 4 to automate the initial conversion, which was then manually edited to sanitize the code.

## FAQ

<details>
    <summary><strong>Who is this for?</strong></summary>
<br/>

People who like time boxing & daily routines and also work in the
terminal frequently. It helps you stay on track by letting you know
what you _said_ you'd be working on at any given moment.

</details>

## License

See [LICENSE](LICENSE).
