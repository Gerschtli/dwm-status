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

  legacyCargoFetcher = true;
  cargoSha256 = "1i3zqz7yqbps684rsz422akbvhrbw6qhrmfarkm4zpi557gx400b";

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  postInstall = ''
    wrapProgram $out/bin/${name} --prefix "PATH" : "${binPath}"
  '';
}
