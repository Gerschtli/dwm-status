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

  cargoSha256 = "0lf9zjax2g398qpsdaqqk38afc38jsk0wfhw7sh23771k556nmr7";
}
