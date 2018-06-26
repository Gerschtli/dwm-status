with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "dwm-status";

  src = ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  cargoSha256 = "13ibcbk8shfajk200d8v2p6y3zfrz5dlvxqfw1zsm630s5dmy6qx";

  postInstall = ''
    wrapProgram $out/bin/${name} \
      --prefix "PATH" : "${alsaUtils}/bin"
  '';
}
