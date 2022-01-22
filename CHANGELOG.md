# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.7.3] - 2022-01-22
### Changed
- Updated external dependencies

## [1.7.2] - 2021-05-08
### Changed
- Updated external dependencies

## [1.7.1] - 2020-07-06
### Fixed
- Fixed an [issue](https://github.com/Gerschtli/dwm-status/issues/11) where only one of multiple batteries was
  recognized
- Fixed an [issue](https://github.com/Gerschtli/dwm-status/issues/13) where battery notification were already sent when
  one battery is low despite the fact that at least one other battery was fully charged

## [1.7.0] - 2020-05-19
### Changed
- Replaced logic for gathering battery status information by external crate [battery](https://crates.io/crates/battery)
  to handle more devices and edge cases

## [1.6.4] - 2020-04-20
### Changed
- Updated external dependencies

## [1.6.3] - 2019-09-23
### Changed
- Improved error messages in case of failed command executions

### Fixed
- Fixes reported [issues](https://github.com/Gerschtli/dwm-status/issues/5) in network feature:
  - Fixes incorrect command line arguments of `dig` call for IPv6 address
  - Log warn message instead of exiting dwm-status on command execution failure
  - Interpret exit codes not equal to 0 as error and print `no_value` option in status bar
  - Decrease timeout and tries options of `dig` commands because they are currently synchronous and with default values
    each of these calls have a total maximum running time of 15 seconds

## [1.6.2] - 2019-07-23
### Changed
- Updated external dependencies

## [1.6.1] - 2019-07-07
### Changed
- Internals: Use new rust edition 2018

### Fixed
- Fixes reported [issue](https://github.com/Gerschtli/dwm-status/issues/3) describing rounding errors in battery
  capacity

## [1.6.0] - 2018-12-27
### Added
- Completely new network feature
- Added a bunch of unit tests to ensure code quality, it's the first step but a long way to go for a reasonable code
  coverage..

### Changed
- Improved command line interface, providing `--help` and `--version` options through `clap` crate

## [1.5.0] - 2018-12-23
### Changed
- The super useless case of having one feature at least twice in the `order` list now throws an exception.. Why would
  anyone ever want to see one feature twice in their status bar?
- Update for 1.31 rust and added more lints to improve code quality
- Big refactoring to provide a better interface to plug in new features and to be able to encapsulate logic better.
- Improved log messages adding timestamp and log level

### Deprecated
- The `debug` config option is deprecated and will be removed in 2.0.0. Log level is set to info by default.

### Fixed
- Always restarting notifier threads after an error occured. This fixes the bug, that the notifier thread of the `audio`
  feature dies after a hibernate or suspend.

## [1.4.1] - 2018-12-01
### Fixed
- Update README according to last changes (icons for battery, `time.update_seconds`)
- Update `PATH` for executable in `default.nix`

## [1.4.0] - 2018-11-25
### Changed
- Big refactoring to improve performance and maintainability
- Errors are only visible via stderr and no longer shown as notifications
- Default value of `time.update_seconds` is automatically set through parsing the `format` string

### Fixed
- Removed useless "notify thread killed" error message on `SIGTERM` when using audio feature

## [1.3.0] - 2018-11-24
### Added
- Dynamic icons for audio, backlight and battery feature (see `icons` setting in the respective feature documentation)

### Fixed
- The `order` setting is now per default empty as documented, previously this value defaulted to `["time"]`

## [1.2.0] - 2018-11-08
### Added
- Handler to update all features after wakeup from suspend

## [1.1.2] - 2018-08-07
### Changed
- Clippy propagates warnings to errors

### Fixed
- Update time feature with second precision after hibernate

## [1.1.1] - 2018-06-28
### Added
- This changelog

## [1.1.0] - 2018-06-27
### Changed
- Send update message for time feature with second precision, for more information see
  [commit 8e3e695](https://github.com/Gerschtli/dwm-status/commit/8e3e6953e299e987432a479af6dab78acd352bb8)

### Fixed
- Configuration option `separator` didn't have any effect, fixed that

## [1.0.0] - 2018-06-26
### Added
- More configuration options, for example:
  - debug output
  - order of features
  - separator string between modules
  - and many other options for all features, for more information see `README.md`

### Breaking Changes
- New format of configuration file

## [0.5.1] - 2018-06-16
### Changed
- Changes feature name of CPU load feature to `cpu_load`

### Fixed
- Flush display after updating window name

## [0.5.0] - 2018-06-15
### Added
- Runtime dependencies in `default.nix`
- Handling of SIGTERM events to close X11 display

### Changed
- Replaced external command `xsetroot` to set window name with direct call of X11 library functions
- Moves `makeWrapper` and `pkgconfig` to `nativeBuildInputs` in `default.nix`
- Documents need of upower in README to use battery feature

## [0.4.0] - 2018-06-09
### Added
- CPU load feature

### Changed
- Allow travis failures for nightly because of too many false alarms caused by `clippy`

### Fixed
- `cargoSha256` in `default.nix` which has to be updated after every dependency update

## [0.3.3] - 2018-05-19
### Changed
- Use `buildRustPackage` for `default.nix`
- Use [`clippy`](https://github.com/rust-lang-nursery/rust-clippy) as cargo subcommand

### Fixed
- Run `cargo check` before `cargo clippy` in Travis because of a bug in chrono or clippy
  (see [chrono issue](https://github.com/chronotope/chrono/issues/246) and
  [clippy issue](https://github.com/rust-lang-nursery/rust-clippy/issues/2760))

## [0.3.2] - 2018-04-22
### Added
- [Nix](https://nixos.org/nix/) support via `default.nix` with [carnix](https://nest.pijul.com/pmeunier/carnix)

## [0.3.1] - 2018-04-18
### Changed
- Updates README for new battery notifier behaviour introduced in [0.3.0]

### Fixed
- Compilation error in rust stable

## [0.3.0] - 2018-04-17
### Changed
- Battery notifier only notifies if all batteries are low

### Fixed
- Battery time estimation is now optional if using multiple batteries
- Backlight value limited to maximum value 100%, sometimes the value is greater than 100.. `¯\_(ツ)_/¯`

## [0.2.0] - 2018-04-02
### Added
- Editorconfig file for consistent coding standards
- Support for multiple batteries
- Support for `energy_*` and `power_*` files in `/sys/class/power_supply`
- More badges in README - Yeah!
- `release.toml` for use with [cargo-release](https://github.com/sunng87/cargo-release)

### Changed
- New `Renderable` trait to replace duplicate `format!` calls

### Fixed
- Update feature after initialization of feature notifier to catch all events

## [0.1.0] - 2018-03-23
### Added
- Initial project with
  - Audio, backlight, battery and time features
  - Travis config with cargo fmt, build and test checks

[Unreleased]: https://github.com/Gerschtli/dwm-status/compare/1.7.3...HEAD
[1.7.3]: https://github.com/Gerschtli/dwm-status/compare/1.7.2...1.7.3
[1.7.2]: https://github.com/Gerschtli/dwm-status/compare/1.7.1...1.7.2
[1.7.1]: https://github.com/Gerschtli/dwm-status/compare/1.7.0...1.7.1
[1.7.0]: https://github.com/Gerschtli/dwm-status/compare/1.6.4...1.7.0
[1.6.4]: https://github.com/Gerschtli/dwm-status/compare/1.6.3...1.6.4
[1.6.3]: https://github.com/Gerschtli/dwm-status/compare/1.6.2...1.6.3
[1.6.2]: https://github.com/Gerschtli/dwm-status/compare/1.6.1...1.6.2
[1.6.1]: https://github.com/Gerschtli/dwm-status/compare/1.6.0...1.6.1
[1.6.0]: https://github.com/Gerschtli/dwm-status/compare/1.5.0...1.6.0
[1.5.0]: https://github.com/Gerschtli/dwm-status/compare/1.4.1...1.5.0
[1.4.1]: https://github.com/Gerschtli/dwm-status/compare/1.4.0...1.4.1
[1.4.0]: https://github.com/Gerschtli/dwm-status/compare/1.3.0...1.4.0
[1.3.0]: https://github.com/Gerschtli/dwm-status/compare/1.2.0...1.3.0
[1.2.0]: https://github.com/Gerschtli/dwm-status/compare/1.1.2...1.2.0
[1.1.2]: https://github.com/Gerschtli/dwm-status/compare/1.1.1...1.1.2
[1.1.1]: https://github.com/Gerschtli/dwm-status/compare/1.1.0...1.1.1
[1.1.0]: https://github.com/Gerschtli/dwm-status/compare/1.0.0...1.1.0
[1.0.0]: https://github.com/Gerschtli/dwm-status/compare/0.5.1...1.0.0
[0.5.1]: https://github.com/Gerschtli/dwm-status/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/Gerschtli/dwm-status/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/Gerschtli/dwm-status/compare/0.3.3...0.4.0
[0.3.3]: https://github.com/Gerschtli/dwm-status/compare/0.3.2...0.3.3
[0.3.2]: https://github.com/Gerschtli/dwm-status/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/Gerschtli/dwm-status/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/Gerschtli/dwm-status/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/Gerschtli/dwm-status/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/Gerschtli/dwm-status/compare/29661b3d5b8b10432f69ac6db8755251298aa5e9...0.1.0
