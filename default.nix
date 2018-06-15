with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  cargoSha256 = "0qr999hwrqn7a4n4kvbrpli7shxp9jchj8csxzsw951qmzq32qwv";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${alsaUtils}/bin"
  '';
}
