{ naersk-lib, pkgs, useGlobalAlsaUtils ? false }:

let
  binPath = pkgs.lib.makeBinPath (
    (with pkgs; [
      coreutils # audio: stdbuf
      dnsutils # network: dig
      iproute # network: ip
      wirelesstools # network: iwgetid
    ])
    ++ pkgs.lib.optional (!useGlobalAlsaUtils) pkgs.alsaUtils # audio: alsactl, amixer
  );

  name = "dwm-status";
in

naersk-lib.buildPackage {
  pname = name;

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  nativeBuildInputs = with pkgs; [ makeWrapper pkgconfig ];
  buildInputs = with pkgs; [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  postInstall = ''
    # run only when building the final package
    if [[ -x $out/bin/${name} ]]; then
      wrapProgram $out/bin/${name} --prefix "PATH" : "${binPath}"
    fi
  '';
}
