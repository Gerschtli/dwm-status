with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
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
