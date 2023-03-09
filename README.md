# `»` rsx

[![Checks](https://img.shields.io/github/actions/workflow/status/norskeld/rsx/check.yml?style=flat-square&colorA=22272d&colorB=22272d&label=checks)](https://github.com/norskeld/rsx/actions)

> `R`usty `S`cripts E`x`ecutor

Micro CLI for interactive execution of npm & yarn scripts.

## Preview

[![asciicast](https://asciinema.org/a/566078.svg)](https://asciinema.org/a/566078)

## Motivation

**First**, I wanted to practise creating Rust command-line apps.

**Secondly**, I'm lazy. Like, _really lazy_. I don't even like to type much. And sometimes I'm perplexed by amount of steps required to run an npm or yarn script. Especially if you can't remember what scripts a project even has without diving into an IDE or `cat`ing (`less`ing, `bat`ing, whatever) a `package.json`.

**Thirdly**, if you seek for autocompletion, I would suggest you to use [this zsh enhancement](https://github.com/lukechilds/zsh-better-npm-completion).

If you are like me—read on.

## Installation

### macOS Intel (Homebrew)

```shell
brew tap norskeld/tap
brew install norskeld/tap/rsx
```

### macOS / Linux / Windows (Cargo)

Make sure to [install Rust toolchain][rust-toolchain] first. After that you can install rsx via **Cargo**:

```shell
cargo install --locked --git https://github.com/norskeld/rsx
```

## Features

### Implemented

You can:

- [x] Interactively select and run scripts from `package.json` in the current working directory:
  - [x] Use arrows _or_ <kbd>J</kbd> / <kbd>K</kbd> keys to select a script.
  - [x] Press <kbd>Ctrl+A</kbd> / <kbd>Home</kbd> to jump to the beginning.
  - [x] Press <kbd>Ctrl+E</kbd> / <kbd>End</kbd> to jump to the end.
  - [x] Press <kbd>Ctrl+C</kbd> / <kbd>Esc</kbd> to abort selection.
- [x] Use different package managers: **npm**, **pnpm**, or **yarn**.
- [x] Override default package manager (**npm**) through env variable.
- [x] Pass a script name directly as argument and run via the selected package manager.
- [x] Pass additional arguments after `--` to a selected script.

### Planned

- [ ] Determine package manager automatically. This can be done using simple heuristics like:
  - Checking the lock files, they all differ between different package managers.
  - Check the `packageManager` field in `package.json`.
  - Briefly check contents of the `node_modules`.
- [ ] Prettify output.
  - [x] Display current package manager.
  - [x] Dim script command.
  - [ ] Properly wrap too long script commands.
  - [ ] Refactor status and "Executing..." message handling and styling.
- [ ] Quickly find scripts by simply typing their name.
- [ ] Select multiple scripts and run them in order.

### Maybe

- [ ] Maybe support monorepos.

## License

[MIT](LICENSE).

<!-- Links -->

[cargo]: https://doc.rust-lang.org/cargo/
[rust-toolchain]: https://www.rust-lang.org/tools/install
