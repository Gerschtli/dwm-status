{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs { overlays = []; }
, useGlobalAlsaUtils ? false
}:

with pkgs;

let
  naersk = callPackage sources.naersk { };

  binPath = stdenv.lib.makeBinPath (
    [
      coreutils     # audio:   stdbuf
      dnsutils      # network: dig
      iproute       # network: ip
      wirelesstools # network: iwgetid
    ]
    ++ lib.optional (!useGlobalAlsaUtils) alsaUtils # audio: alsactl, amixer
  );
in

naersk.buildPackage {
  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  postInstall = ''
    # run only when building the final package
    if [[ -x $out/bin/dwm-status ]]; then
      wrapProgram $out/bin/dwm-status --prefix "PATH" : "${binPath}"
    fi
  '';
}
