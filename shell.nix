{ pkgs }:

let
  buildInputs = with pkgs; [
    # build dependencies
    dbus
    gdk-pixbuf
    glib
    libnotify
    pkg-config
    xorg.libX11

    # run-time dependencies
    alsa-utils
    coreutils
    dnsutils
    iproute2
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
in

pkgs.mkShell {
  inherit buildInputs;

  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";

  # RUST_BACKTRACE = 1;
}
