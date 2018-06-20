with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  cargoSha256 = "0aq9rnfy76cqzc48n2br3fj1llapxib95zd92q9gfyxvwp5ay58a";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${alsaUtils}/bin"
  '';
}
