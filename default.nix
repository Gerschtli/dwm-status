with import <nixpkgs> { };

let
  runtimeDeps = [ alsaUtils xorg.xsetroot ];
in

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = ./.;

  buildInputs = [
    dbus
    gdk_pixbuf
    libnotify
    makeWrapper
    pkgconfig
    xorg.libX11
  ];

  cargoSha256 = "1xbk3ly52qx99pjxnl6fmkax5mkxswq9acajpmjkx4cm8ljws0h8";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${stdenv.lib.makeBinPath runtimeDeps}"
  '';
}
