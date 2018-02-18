# dwm-status

DWM status service which dynamically updates on DBus messages.

Build and run with:
```sh
$ cargo run
```

Sets status via `xsetroot` in this pattern:
```
[ <backlight> / ] <sound> / [ <battery> / ] <time>
```

E.g.:
```
L: 20% / S: 50% / - 87% (03:50) / 2018-01-01 13:37
MUTE / NO BATT / 2018-01-01 13:37
```
