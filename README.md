# dwm-status [![Travis CI](https://img.shields.io/travis/Gerschtli/dwm-status.svg?style=flat-square)](https://travis-ci.org/Gerschtli/dwm-status) [![Crates.io](https://img.shields.io/crates/v/dwm-status.svg?style=flat-square)](https://crates.io/crates/dwm-status) [![The MIT License](https://img.shields.io/badge/license-MIT-orange.svg?style=flat-square)](http://opensource.org/licenses/MIT)

DWM status service which dynamically updates when needed.
Heavily inspired by [i3status-rust](https://github.com/greshake/i3status-rust).

Example status bar:
```
L 30% / MUTE / ▼ 25% (01:05) / 2018-03-16 21:25
```

If an error occures, notifications via libnotify are sent.

## Requirements

`cargo`, `rustc` and `lib{dbus,gdk-pixbuf,glib,notify}-dev` are required to build the binary.
To set the status text `xsetroot` is used.

The `upower` daemon has to be running for the battery feature for receiving DBus messages.

## Build and run

```sh
$ # dev mode
$ cargo run -- <config-file>
$ # release mode
$ cargo build --release
$ ./target/release/dwm-status <config-file>
```
Or install globally to `~/.cargo/bin`:
```sh
$ cargo install
```

## Configuration

The `config-file` contains one `feature` per line, e.g.:
```
audio
battery
time
```

### Features

#### Audio

Shows either `MUTE` or the current volume like `S 52%`. Listens on `alsactl monitor` for changes.

alsa-utils are required.

#### Backlight

Shows backlight value like `L 23%` and watches `/sys/class/backlight` for changes.

#### Battery

Shows following information:

| status               | example         | notes                        |
| -------------------- | --------------- | ---------------------------- |
| charging             | `▲ 10% (01:23)` | In parentheses time to full  |
| discharging          | `▼ 50% (02:03)` | In parentheses time to empty |

| battery count | example                       | notes                                    |
| ------------- | ----------------------------- | ---------------------------------------- |
| 0             | `NO BATT`                     |                                          |
| 1             | `▼ 50% (02:03)`               |                                          |
| 2             | `▼ 50% (02:03) · 50% (02:03)` | Batteries ordered alphabetically by name |

Watches UPower DBus signals for added or removed batteries and changes of battery states.

If discharging and the capacity is under specific values, warning notifications with urgency normal or critical are sent.

| capacity | urgency  |
| -------- | -------- |
| 2%       | critical |
| 5%       | critical |
| 10%      | critical |
| 15%      | normal   |
| 20%      | normal   |

#### Time

Shows time in format `YYYY-MM-DD HH:MM` and refreshes every minute.
