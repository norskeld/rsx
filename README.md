# `»` rsx

[![Checks](https://img.shields.io/github/workflow/status/norskeld/rsx/check?style=flat-square&colorA=22272d&colorB=22272d&label=checks)](https://github.com/norskeld/rsx/actions)

> **R**usty **S**cripts E**x**ecutor

Micro CLI for interactive execution of npm & yarn scripts.

## Preview

[![asciicast](https://asciinema.org/a/499299.svg)](https://asciinema.org/a/499299)

## Installation

You can install CLI with [cargo]. `rustc` must be installed as well, preferably the latest **stable**.

```bash
cargo install --locked --git https://github.com/norskeld/rsx
```

This will fetch the repository, compile (respecting the `Cargo.lock`, hence that `--locked` flag), and install a global binary named `sx`.

## Features

Basic:

- [x] Interactively select and run scripts from `package.json` in the current working directory:
  - [x] Use arrows *or* <kbd>J</kbd> / <kbd>K</kbd> keys to select a script.
  - [x] Press <kbd>Ctrl+A</kbd> / <kbd>Home</kbd> to jump to the beginning.
  - [x] Press <kbd>Ctrl+E</kbd> / <kbd>End</kbd> to jump to the end.
  - [x] Press <kbd>Ctrl+C</kbd> / <kbd>Esc</kbd> to abort selection.
- [x] Use different package managers: **npm**, **pnpm**, or **yarn**.
- [x] Override default package manager (**npm**) through env variable.

Planned:

- [ ] Prettify output.
- [ ] Quickly find scripts by simply typing their name.
- [ ] Select multiple scripts and run them in order.
- [x] Pass a script name directly as argument.

Maybe:

- [ ] Maybe support `packageManager` field in `package.json`.
- [ ] Maybe support monorepos.

## Motivation

**First**, I wanted to practise creating Rust command-line apps.

**Secondly**, I'm lazy. Like, _really lazy_. I don't even like to type much. And sometimes I'm perplexed by amount of steps required to run an npm or yarn script. Especially if you can't remember what scripts a project even has without diving into an IDE or `cat`ing (`less`ing, `bat`ing, whatever) a `package.json`.

**Thirdly**, if you seek for autocompletion, I would suggest you to use [this zsh enhancement](https://github.com/lukechilds/zsh-better-npm-completion).
If you are like me—read on.

## License

[MIT](LICENSE).

<!-- Links -->

[cargo]: https://doc.rust-lang.org/cargo/
