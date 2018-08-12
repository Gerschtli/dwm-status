# dwm-status [![Travis CI](https://img.shields.io/travis/Gerschtli/dwm-status.svg?style=flat-square)](https://travis-ci.org/Gerschtli/dwm-status) [![Crates.io](https://img.shields.io/crates/v/dwm-status.svg?style=flat-square)](https://crates.io/crates/dwm-status) [![The MIT License](https://img.shields.io/badge/license-MIT-orange.svg?style=flat-square)](http://opensource.org/licenses/MIT)

DWM status service which dynamically updates when needed.
Heavily inspired by [i3status-rust](https://github.com/greshake/i3status-rust).

Example status bar:
```
L 30% / MUTE / ▼ 25% (01:05) / 2018-03-16 21:25
```

If an error occurs, notifications are sent via libnotify.

## Requirements

`cargo`, `rustc` and `lib{dbus,gdk-pixbuf,notify,x11}-dev` are required to build the binary.

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
$ cargo install dwm-status
```

### [Nix](https://nixos.org/nix/) support

Build:
```sh
$ nix-build
```

And run:
```sh
$ ./result/bin/dwm_status <config-file>
```

## Configuration

The `config-file` can be a HJSON, JSON, TOML or YAML file. See in `examples/settings` for how examples filled with the
default values. The type of file is determined by its suffix.

### General options

| name        | default | description                                      |
| ----------- | ------- | ------------------------------------------------ |
| `debug`     | `false` | Whether to print to stdout when the bar updates. |
| `order`     | `[]`    | List of enabled features in order.               |
| `separator` | `" / "` | Separator string between each feature.           |

### Feature: Audio

**Note:** `alsa-utils` are required.

Shows status of configured alsa control device. Listens on `alsactl monitor` for changes.

#### Configuration options

| name       | default      | description                                                                     |
| ---------- | ------------ | ------------------------------------------------------------------------------- |
| `control`  | `"Master"`   | Alsa control device to listen for.                                              |
| `mute`     | `"MUTE"`     | Text representation if muted.                                                   |
| `template` | `"S {VOL}%"` | Text representation if unmuted. (`{VOL}` gets replaced with the current volume) |

### Feature: Backlight

Shows status of backlight value and watches `/sys/class/backlight` for changes.

#### Configuration options

| name       | default             | description                                                                  |
| ---------- | ------------------- | ---------------------------------------------------------------------------- |
| `device`   | `"intel_backlight"` | Backlight device in `/sys/class/backlight`.                                  |
| `template` | `"L {BL}%"`         | Text representation. (`{BL}` gets replaced with the current backlight value) |

### Feature: Battery

**Note:** The `upower` daemon has to be running for receiving DBus messages.

Watches UPower DBus signals for added or removed batteries and changes of battery states.

If notifier is enabled, currently discharging and the capacity of the fullest battery is under the configured values
value, warning notifications with urgency normal or critical (depending on the configured critical value) are sent.

Shows following information per battery:

| status               | example         | notes                        |
| -------------------- | --------------- | ---------------------------- |
| charging             | `▲ 10% (01:23)` | In parentheses time to full  |
| discharging          | `▼ 50% (02:03)` | In parentheses time to empty |

Shows following information as feature block:

| battery count | example               | notes                                    |
| ------------- | --------------------- | ---------------------------------------- |
| 0             | `NO BATT`             |                                          |
| 1             | `▼ 50% (02:03)`       |                                          |
| 2             | `▼ 50% (02:03) · 50%` | Batteries ordered alphabetically by name |

#### Configuration options

| name                | default              | description                                                |
| ------------------- | -------------------- | ---------------------------------------------------------- |
| `charging`          | `"▲"`                | Text representation for status charging.                   |
| `discharging`       | `"▼"`                | Text representation for status discharging.                |
| `enable_notifier`   | `true`               | Whether to enable the notifier.                            |
| `no_battery`        | `"NO BATT"`          | Text representation if no battery present.                 |
| `notifier_critical` | `10`                 | Maximum battery value to notify via critical notification. |
| `notifier_levels`   | `[2, 5, 10, 15, 20]` | Battery values to notify.                                  |
| `separator`         | `" · "`              | Separator string between mutliple battery infos.           |

### Feature: CPU Load

Shows CPU load taken from `/proc/loadavg` in configured format and refreshes every `n` seconds.

#### Configuration options

| name              | default                | description                 |
| ----------------- | ---------------------- | --------------------------- |
| `template`        | `"{CL1} {CL5} {CL15}"` | Text representation. (`{CLx}` gets replaced with the load<br/>of last `x` minutes for `x` in `{1, 5, 15}`) |
| `update_interval` | `20`                   | Update interval in seconds. |

### Feature: Time

Shows time in configured format and refreshes every second or minute.

#### Configuration options

| name             | default             | description                                                          |
| ---------------- | ------------------- | -------------------------------------------------------------------- |
| `format`         | `"%Y-%m-%d %H:%M"`  | Time format of [chrono crate](https://github.com/chronotope/chrono). |
| `update_seconds` | `false`             | Whether to update time feature every second or minute.               |

## Contributing

You need `rustup` with nightly toolchain, rustfmt, clippy and `lib{dbus,gdk-pixbuf,notify,x11}-dev`. I recommend the
installation of racer.

If your are using [nix](https://nixos.org/nix) you can use `shell.nix` for all dependencies except the `rustup`
toolchain and components:

```sh
$ nix-shell
[nix-shell]$ rustup install nightly
[nix-shell]$ rustup default nightly
[nix-shell]$ rustup component add clippy-preview
[nix-shell]$ rustup component add rustfmt-preview
```
