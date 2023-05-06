# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Lots of code refactoring
- New `[minecraft]` optional section with `version` and `loader` properties in manifest file

  - `add` command now filters versions based on these properties

- `name` property in mod file is now required
- Add `remove` subcommand to remove mods
- Add modrinth slug and id checking
- Remove unused `futures` crate

## [0.1.0-rc.0] - 2023-04-12

Initial release

[Unreleased]: https://github.com/panda885/niter/compare/v0.1.0-rc.0...HEAD
[0.1.0-rc.0]: https://github.com/panda885/niter/releases/tag/v0.1.0-rc.0