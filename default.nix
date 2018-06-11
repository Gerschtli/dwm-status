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
  ];

  cargoSha256 = "0169k91pb7ipvi0m71cmkppp1klgp5ghampa7x0fxkyrvrf0dvqg";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${stdenv.lib.makeBinPath runtimeDeps}"
  '';
}
