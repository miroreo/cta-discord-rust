{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    crate2nix.url = "github:nix-community/crate2nix";
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    ...
  }: let
    perSystem = flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      imports = [
        ./devshell.nix
      ];

      flake = {system, ...}: rec {
        overlays.default = final: prev: {
          cta-discord = self.packages."${final.system}".default;
        };
        nixosModules.default = import ./module.nix;
      };

      perSystem = {
        system,
        pkgs,
        lib,
        inputs',
        ...
      }: let
        # If you dislike IFD, you can also generate it with `crate2nix generate`
        # on each dependency change and import it here with `import ./Cargo.nix`.
        cargoNix = import ./Cargo.nix {pkgs = pkgs;};
      in rec {
        checks = {
          rustnix = cargoNix.rootCrate.build.override {
            runTests = true;
          };
        };

        packages = {
          rustnix = cargoNix.rootCrate.build;
          default = packages.rustnix;

          inherit (pkgs) rust-toolchain;

          rust-toolchain-versions = pkgs.writeScriptBin "rust-toolchain-versions" ''
            ${pkgs.rust-toolchain}/bin/cargo --version
            ${pkgs.rust-toolchain}/bin/rustc --version
          '';
        };
      };
    };
  in
    perSystem
    // {
      hydraJobs.default."x86_64-linux" = perSystem.packages.x86_64-linux.default;
    };
}
