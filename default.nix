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
  name = "dwm-status";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  cargoSha256 = "02wfgc9vnpl1326lwkal8hhd0lmfxgmrj8shvpwxkwjf0psvj4ph";

  postInstall = ''
    wrapProgram $out/bin/${name} --prefix "PATH" : "${binPath}"
  '';
}
