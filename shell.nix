with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    dbus
    gdk_pixbuf
    latest.rustChannels.nightly.rust
    libnotify
    pkgconfig
    rustracer
  ];

  RUST_BACKTRACE = 1;
}
