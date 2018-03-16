with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    cargo-edit
    rustracer
    dbus
    gdk_pixbuf
    glib
    latest.rustChannels.nightly.rust
    libnotify
    pkgconfig
  ];
}
