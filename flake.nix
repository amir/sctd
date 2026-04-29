{
  description = "sctd — set color temperature daemon";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    let
      mkSctd =
        pkgs:
        pkgs.callPackage ./nix/package.nix {
          src = self;
        };
    in
    {
      overlays.default = final: _prev: {
        sctd = mkSctd final;
      };

      homeManagerModules.sctd = import ./nix/hm-module.nix;
      homeManagerModules.default = self.homeManagerModules.sctd;
    }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        sctd = mkSctd pkgs;
      in
      {
        packages.sctd = sctd;
        packages.default = sctd;

        apps.default = flake-utils.lib.mkApp { drv = sctd; };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ sctd ];
          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
          ];
        };

        checks.sctd = sctd;
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
