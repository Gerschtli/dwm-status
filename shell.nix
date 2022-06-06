{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # build dependencies
    dbus
    gdk-pixbuf
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
    nixpkgs-fmt
    rustup

    # tarpaulin
    # run RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin -f
    openssl
    zlib
  ];

  # RUST_BACKTRACE = 1;
}
