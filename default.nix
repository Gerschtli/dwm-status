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

  cargoSha256 = "0wbbbk99hxxlrkm389iqni9aqvw2laarwk6hhwsr4ph3y278qhi8";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${binPath}"
  '';
}
