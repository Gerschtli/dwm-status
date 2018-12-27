with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    cargo-edit
    cargo-release
    dbus
    dnsutils
    gdk_pixbuf
    iproute
    libnotify
    pkgconfig
    rustracer
    rustup
    wirelesstools
    xorg.libX11
  ];

  # RUST_BACKTRACE = 1;
}
