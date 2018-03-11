with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    cargo-edit
    dbus
    gdk_pixbuf
    glib
    latest.rustChannels.nightly.cargo
    latest.rustChannels.nightly.rust
    libnotify
    pkgconfig
  ];
}
