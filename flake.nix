{
  description = "Highly performant and configurable DWM status service";

  inputs = {
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
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
      packages.${system} = {
        default = package;
        ${name} = package;
        ${nameWithGlobalAlsaUtils} = import ./. {
          inherit naersk-lib pkgs;
          useGlobalAlsaUtils = true;
        };
      };

      apps.${system} = {
        default = app;
        ${name} = app;
      };

      overlays.default = final: prev:
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

      devShells.${system}.default = import ./shell.nix { inherit pkgs; };
    };
}
