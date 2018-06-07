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

  cargoSha256 = "04bs7pwmbi52agjn1r549cws5i7c5abhi7pkb5lzzkpszknprv5v";
}
