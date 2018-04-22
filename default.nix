with import <nixpkgs> { };

((import ./dwm-status.nix).dwm_status {}).override {
  crateOverrides = defaultCrateOverrides // {
    dwm-status = attrs: {
      buildInputs = [
        libnotify
      ];
    };

    gdk-pixbuf-sys = attrs: {
      buildInputs = [
        gdk_pixbuf
        pkgconfig
      ];
    };
  };
}
