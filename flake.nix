{
  description = "A local CLI for logging small agent workflow papercuts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    systems.url = "github:nix-systems/triplet";
  };

  outputs =
    {
      self,
      crane,
      nixpkgs,
      systems,
    }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = eachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          craneLib = crane.mkLib pkgs;
          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            cargoExtraArgs = "--bin papercut";
            doCheck = false;
            strictDeps = true;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          papercut = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );
        in
        {
          default = papercut;
          inherit papercut;
        }
      );

      checks = eachSystem (system: {
        inherit (self.packages.${system}) papercut;
      });

      apps = eachSystem (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.papercut}/bin/papercut";
        };
      });

      devShells = eachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          craneLib = crane.mkLib pkgs;
        in
        {
          default = craneLib.devShell {
            checks = self.checks.${system};
          };
        }
      );
    };
}
