{ pkgs ? import <nixpkgs> { }, useGlobalAlsaUtils ? false }:

with pkgs;

let
  binPath = stdenv.lib.makeBinPath (
    [
      coreutils     # audio:   stdbuf
      dnsutils      # network: dig
      iproute       # network: ip
      wirelesstools # network: iwgetid
    ]
    ++ pkgs.lib.optional (!useGlobalAlsaUtils) alsaUtils # audio: alsactl, amixer
  );
in

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  cargoSha256 = "0fk9fxzq4kjjr7pzfhf3j1bc2zms87hk7fcxxw49ybdkniazx9k5";

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  postInstall = ''
    wrapProgram $out/bin/${name} --prefix "PATH" : "${binPath}"
  '';
}
