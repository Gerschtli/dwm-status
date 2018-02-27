with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    cargo-edit
    glib
    gdk_pixbuf
    latest.rustChannels.stable.cargo
    latest.rustChannels.stable.rust
    pkgconfig
  ];
}
