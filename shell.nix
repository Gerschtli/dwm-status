let
  sources = import ./nix/sources.nix;
  niv = (import sources.niv { }).niv;
  pkgs = import sources.nixpkgs { overlays = [ ]; };
in

pkgs.mkShell {
  buildInputs = with pkgs; [
    # build dependencies
    dbus
    gdk_pixbuf
    libnotify
    pkgconfig
    xorg.libX11
    xcb-xkb

    # run-time dependencies
    alsaUtils
    coreutils
    dnsutils
    iproute
    wirelesstools

    # dev tools
    cargo-edit
    cargo-release
    niv
    rustup

    # tarpaulin
    # run RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin -f
    openssl
    zlib
  ];

  # RUST_BACKTRACE = 1;
}
