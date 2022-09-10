{
  description = "Rust application and setup env";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, naersk, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";

        toolchain = fenix.packages."${system}".stable;

        naersk-lib = naersk.lib."${system}".override {
          cargo = toolchain.cargo;
          rustc = toolchain.rustc;
        };

        manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        version = manifest.package.version;

        tmgr = naersk-lib.buildPackage {
          inherit version;
          pname = "tmgr";
          root = ./.;

          buildInputs = with pkgs; [ ];
          nativeBuildInputs = with pkgs; [ ];
        };

        devShell = pkgs.mkShell {
          name = "tmgr";
          packages = with pkgs; with toolchain; [
            # Core rust
            rustc
            cargo
            rust-src

            # Development tools
            rust-analyzer
            rustfmt-preview
            clippy-preview

            # Cargo extensions
            cargo-bloat
            cargo-edit
            cargo-license
            cargo-limit
            cargo-watch
            cargo-whatfeatures
          ] ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin [
            libiconv
          ]);

          CARGO_BUILD_RUSTFLAGS = if pkgs.stdenv.isDarwin then "-C rpath" else null;
          RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library";
        };
      in
      rec {
        inherit devShell;

        # `nix build`
        packages.tmgr = tmgr;
        defaultPackage = self.packages.${system}.tmgr;

        # `nix run`
        apps.tmgr = inputs.flake-utils.lib.mkApp { drv = packages.tmgr; };
        defaultApp = apps.tmgr;
      });
}

