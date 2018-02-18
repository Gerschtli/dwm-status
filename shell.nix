with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "dwm-status";

  buildInputs = [
    cargo-edit
    dbus
    latest.rustChannels.stable.cargo
    latest.rustChannels.stable.rust
    pkgconfig
  ];
}
