# niter

> **A modern command-line tool for creating Minecraft modpacks**

[![Crates.io](https://img.shields.io/crates/v/niterpack?style=flat-square)](https://crates.io/crates/niterpack)
[![Build](https://img.shields.io/github/actions/workflow/status/panda885/niter/ci.yml?branch=main&style=flat-square)](https://github.com/panda885/niter/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/niterpack?style=flat-square)](https://crates.io/crates/niterpack)

**🛠️ Heavily under development - release candidate versions only**

Niter is a modern command-line tool for creating Minecraft modpacks.
It uses TOML format, which allows the modpack to be easily version-controlled using [git](https://git-scm.com/) or other tools.
Because of the command-line nature, it can also be implemented with continuous integration.
Its interface is easy to use and is inspired by Rust's package manager [Cargo](https://github.com/rust-lang/cargo).

## Features

- Easy to use command-line interface
- TOML format, which can be version-controlled
- Adding mods from [Modrinth](https://modrinth.com/)
- Building modpacks into a usable installation

More features are yet to be added, and you can request more using [the issue tracker](https://github.com/panda885/niter/issues).

## Install

Currently, you can download pre-built binaries from [the GitHub release page](https://github.com/panda885/niter/releases/latest) or install using [Cargo](https://github.com/rust-lang/cargo):

```sh
cargo install niterpack
```

## Usage

Create a new modpack using the `init` command:

```sh
niter init # Creates a new project in the current directory
```

Add your favorite mods from [Modrinth](https://modrinth.com/):

```sh
niter add <MOD> # Adds a new mod to the current project
```

Build the project:

```sh
niter build # Builds the current project
```

Your modpack is now available under `build/installation`.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/panda886/niter/blob/main/LICENSE-APACHE) or [The MIT license](https://github.com/panda885/niter/blob/main/LICENSE-MIT) at your option.