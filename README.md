# dwm-status

DWM status service which dynamically updates on DBus messages.
Heavily inspired by [i3status-rust](https://github.com/greshake/i3status-rust).

Build and run with:
```sh
$ cargo run -- <features>+
```

With `features` of list:

 * `audio` (e.g. `MUTE` / `S 52%`)
 * `backlight` (e.g. `L 23%`)
 * `battery` (e.g. `+ 10% (01:23)` / `- 50% (02:03)` / `= 100%` / `NO BATT`)
 * `time` (e.g. `2018-01-01 13:37`)
