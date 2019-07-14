{ cargoSha256 ? "102vr7xprz1sfsmd725kbykm4lk8p2lx1mv8kvs84n7clx0xcg2g" }:

with import <nixpkgs> { };

let
  binPath = stdenv.lib.makeBinPath [
    alsaUtils     # audio:   alsactl, amixer
    coreutils     # audio:   stdbuf
    dnsutils      # network: dig
    iproute       # network: ip
    wirelesstools # network: iwgetid
  ];
in

rustPlatform.buildRustPackage rec {
  inherit cargoSha256;

  name = "dwm-status";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  postInstall = ''
    wrapProgram $out/bin/${name} --prefix "PATH" : "${binPath}"
  '';
}
