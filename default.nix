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

  cargoSha256 = "1khknf1bjs80cc2n4jnpilf8cc15crykhhyvvff6q4ay40353gr6";

  postInstall = ''
    wrapProgram $out/bin/${name} --prefix "PATH" : "${binPath}"
  '';
}
