let
  sources = import ./nix/sources.nix;
  nixpkgs-mozilla = import sources.nixpkgs-mozilla;
  niv = import sources.niv;
  pkgs = import sources.nixpkgs {
    overlays =
      [
        nixpkgs-mozilla
        (self: super:
          {
            rustc = self.latest.rustChannels.nightly.rust;
            cargo = self.latest.rustChannels.nightly.rust;
          }
        )
      ];
  };
in

pkgs.mkShell {
  buildInputs = with pkgs; [
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
    cargo
    cargo-edit
    cargo-release
    niv
    rustc

    # tarpaulin
    # run RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin -f
    openssl
    zlib
  ];

  # RUST_BACKTRACE = 1;
}
