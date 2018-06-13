with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  cargoSha256 = "1xbk3ly52qx99pjxnl6fmkax5mkxswq9acajpmjkx4cm8ljws0h8";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${alsaUtils}/bin"
  '';
}
