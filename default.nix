with import <nixpkgs> { };

let
  binPath = stdenv.lib.makeBinPath [
    alsaUtils
    bash
    coreutils
  ];
in

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  cargoSha256 = "0f81hj7znszrgsadic810cfpg1qygkdl4vv814y37fm6589cw13w";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${binPath}"
  '';
}
