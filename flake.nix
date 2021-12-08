{
  description = "Highly performant and configurable DWM status service";

  inputs = {
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-21.11";
  };

  outputs = { self, naersk, nixpkgs }:
    let
      system = "x86_64-linux";
      naersk-lib = naersk.lib.${system};
      pkgs = nixpkgs.legacyPackages.${system};

      package = import ./. {
        inherit naersk-lib pkgs;
      };

      name = package.pname;
      nameWithGlobalAlsaUtils = "${name}-global-alsa-utils";

      app = {
        type = "app";
        program = "${package}/bin/${name}";
      };
    in
    {
      defaultPackage.${system} = package;

      packages.${system} = {
        ${name} = package;
        ${nameWithGlobalAlsaUtils} = import ./. {
          inherit naersk-lib pkgs;
          useGlobalAlsaUtils = true;
        };
      };

      defaultApp.${system} = app;
      apps.${system}.${name} = app;

      overlay = final: prev:
        let
          args = {
            naersk-lib = (naersk.overlay final prev).naersk;
            pkgs = prev;
          };
        in
        {
          ${name} = import ./. args;
          ${nameWithGlobalAlsaUtils} = import ./. (args // { useGlobalAlsaUtils = true; });
        };

      devShell.${system} = import ./shell.nix { inherit pkgs; };
    };
}
