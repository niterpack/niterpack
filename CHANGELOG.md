# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Add a `config` directory that contains mod configurations

  - This directory is included in builds

- Rename `installation` to `instance`
- Only mods that have been changed will be updated

## [0.1.0-rc.1] - 2023-05-13

- Manifest file renamed to `niterpack.toml`
- `build` command now uses first file if primary file is not defined
- Fixed `build` requesting version multiple times
- `add` command now uses version number instead of id
- Lots of code refactoring
- New `[minecraft]` optional section with `version` and `loader` properties in manifest file

  - `add` and `build` command now filters versions based on these properties

- `name` property in mod file is now required
- Add `remove` subcommand to remove mods
- Add modrinth slug and id checking
- Remove unused `futures` crate

## [0.1.0-rc.0] - 2023-04-12

Initial release

[Unreleased]: https://github.com/panda885/niter/compare/v0.1.0-rc.1...HEAD
[0.1.0-rc.1]: https://github.com/panda885/niter/compare/v0.1.0-rc.0...v0.1.0-rc.1
[0.1.0-rc.0]: https://github.com/panda885/niter/releases/tag/v0.1.0-rc.0
