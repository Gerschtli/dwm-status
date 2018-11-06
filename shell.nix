with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    cargo-edit
    cargo-release
    dbus
    gdk_pixbuf
    libnotify
    pkgconfig
    rustracer
    rustup
    xorg.libX11
  ];

  RUST_BACKTRACE = 1;
}
