{
  description = "Advent of Code 2024 solutions";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:jkaye2012/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      fenix,
      nixpkgs,
      devenv,
      naersk,
    }:
    devenv.lib.forAllSystems nixpkgs (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenix' = fenix.packages.${system};
        naersk' = pkgs.callPackage naersk {
          cargo = fenix'.complete.toolchain;
          rustc = fenix'.complete.toolchain;
        };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;

        aoc = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-aoc";
          version = "0.3.7";

          buildInputs = with pkgs; [ openssl ];

          nativeBuildInputs = with pkgs; [ pkg-config ];

          src = pkgs.fetchFromGitHub {
            owner = "gobanos";
            repo = pname;
            rev = version;
            hash = "sha256-k9Lm91+Bk6EC8+KfEXhSs4ki385prZ6Vbs6W+18aZSI=";
          };

          cargoHash = "sha256-DKP9YMbVojK7w5pkX/gok4PG6WUjhqUdvTwSir05d0s=";
        };
      in
      {
        devShells.${system}.default = pkgs.mkShell {
          inherit (manifest) name;

          inputsFrom = [ devenv.devShells.${system}.default ];

          packages = with pkgs; [
            aoc
            fenix'.complete.toolchain
            linuxPackages_latest.perf
            lldb
          ];
        };

        packages.${system}.default = naersk'.buildPackage {
          src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
        };
      }
    );
}
