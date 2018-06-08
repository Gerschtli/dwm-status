with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = ./.;

  buildInputs = [
    dbus
    gdk_pixbuf
    libnotify
    pkgconfig
  ];

  cargoSha256 = "0169k91pb7ipvi0m71cmkppp1klgp5ghampa7x0fxkyrvrf0dvqg";
}
