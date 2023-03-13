{
  description = "Tmux utility for session and window management";
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
        root = ./.;

        tmgr = naersk-lib.buildPackage {
          inherit version root;
          pname = "tmgr";

          buildInputs = with pkgs; [ ];
          nativeBuildInputs = with pkgs; [ pandoc ];

          # overrideMain = _: {
          #   postInstall = ''
          #     mkdir -p $out/{completions,man}
          #     OUT_DIR=$out/completions ${tmgrCmpl}/bin/tmgr-cmpl
          #     pandoc --standalone --to man doc/tm.md -o $out/man/tm.1
          #   '';
          # };

          meta = with pkgs.lib; {
            description = "Tmux utility for session and window management";
            homepage = "https://github.com/EdenEast/tmgr";
            license = with licenses; [ mit ];
          };
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
            rustfmt
            clippy

            # Cargo extensions
            cargo-audit
            cargo-bloat
            cargo-edit
            cargo-license
            cargo-limit
            cargo-whatfeatures

            just
            pandoc
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
