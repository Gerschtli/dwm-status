with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    # build dependencies
    dbus
    gdk_pixbuf
    libnotify
    pkgconfig
    xorg.libX11

    # run-time dependencies
    alsaUtils
    coreutils
    dnsutils
    iproute
    wirelesstools

    # dev tools
    cargo-edit
    cargo-release
    rustup

    # tarpaulin
    # run RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin -f
    openssl
    zlib
  ];

  # RUST_BACKTRACE = 1;
}
