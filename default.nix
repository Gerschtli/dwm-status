with import <nixpkgs> { };

let
  binPath = stdenv.lib.makeBinPath [
    alsaUtils bash coreutils
  ];
in

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  cargoSha256 = "0yln64zjccvsp51cvb91zdlra9wra0jw75d7lh2ibr4y4vzmj5yp";

  postInstall = ''
    wrapProgram $out/bin/${name} --prefix "PATH" : "${binPath}"
  '';
}
