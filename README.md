# dwm-status [![Build Status](https://travis-ci.org/Gerschtli/dwm-status.svg?branch=master)](https://travis-ci.org/Gerschtli/dwm-status)

DWM status service which dynamically updates when needed.
Heavily inspired by [i3status-rust](https://github.com/greshake/i3status-rust).

Build and run with:
```sh
$ cargo run -- <config-file>
```

The `config-file` contains one `feature` per line.

With `features` of list:

 * `audio` (e.g. `MUTE` / `S 52%`)
 * `backlight` (e.g. `L 23%`)
 * `battery` (e.g. `+ 10% (01:23)` / `- 50% (02:03)` / `= 100%` / `NO BATT`)
 * `time` (e.g. `2018-01-01 13:37`)
