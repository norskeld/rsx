# `»` rsx

[![Checks](https://img.shields.io/github/workflow/status/norskeld/rsx/check?style=flat-square&colorA=22272d&colorB=22272d&label=checks)](https://github.com/norskeld/rsx/actions)

> **R**usty **S**cripts E**x**ecutor

Micro CLI for interactive execution of npm & yarn scripts.

## Motivation

**First**, I wanted to practise creating Rust apps.

**Secondly**, I'm lazy. Like, _really lazy_. I don't even like to type much. And sometimes I'm perplexed by amount of steps required to run an npm or yarn script. Especially if you can't remember what scripts a project even has without diving into an IDE or `cat`ing (`less`ing, `bat`ing, whatever) a `package.json`.

**Thirdly**, if you seek for autocompletion, I would suggest you to use [this zsh enhancement](https://github.com/lukechilds/zsh-better-npm-completion).
If you are like me—read on.

## Features

Basic:

- [x] Interactively select and run scripts from `package.json` in the current working directory.
- [x] Use different package managers: **npm**, **pnpm**, or **yarn**.
- [x] Override default package manager (**npm**) through env variable.

Planned:

- [ ] Prettify output.
- [ ] Quickly find scripts by simply typing their name.
- [ ] Select multiple scripts and run them in order.
- [ ] Pass a script name (or names) directly as argument.

Maybe:

- [ ] Maybe support `packageManager` field in `package.json`.
- [ ] Maybe support monorepos.

## License

[MIT](LICENSE).
