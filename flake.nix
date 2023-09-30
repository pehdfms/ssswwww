{
  description = "Development Environment and Packaging for SSSWWWW in a Nix Flake";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      inherit (cargoToml.package) name version;
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
    };
  });
}
