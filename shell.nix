with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    cargo-edit
    gdk_pixbuf
    glib
    latest.rustChannels.stable.cargo
    latest.rustChannels.stable.rust
    libnotify
    pkgconfig
  ];
}
