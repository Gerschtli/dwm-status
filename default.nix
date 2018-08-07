with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  cargoSha256 = "1ngdzzxnv4y6xprmkawf6s2696zgwiwgb6ykj5adb4knlx5c634d";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${alsaUtils}/bin"
  '';
}
