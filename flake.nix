{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.flake-compat.follows = "";
      inputs.rust-overlay.follows = "";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        version = manifest.package.version;
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        inherit (pkgs) lib;
        craneLib = crane.lib.${system};

        # Common configuration needed for crane to build the rust project
        args = {
          src = ./.;

          # This is not required as this would just compile the project again
          doCheck = false;
          buildInputs = with pkgs; [
            libiconv
            openssl
          ]
          ++ (lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ]);
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        # Build *just* the cargo dependencies, so we can reuse all of that work between runs
        # This also makes sure that the `build.rs` file is built. If buildPackage is just called
        # the build.rs file was not being executed.
        cargoArtifacts = craneLib.buildDepsOnly args;

        tuxmux = craneLib.buildPackage (args // {
          inherit cargoArtifacts;

          doCheck = true;

          nativeBuildInputs = with pkgs; [
            # Needed for installing shell completions and manpages
            installShellFiles
            pkg-config
          ];

          preFixup = ''
            installManPage target/man/*
            installShellCompletion --bash target/completions/tm.bash
            installShellCompletion --zsh target/completions/_tm
            installShellCompletion --fish target/completions/tm.fish
          '';

          meta = with pkgs.lib; {
            description = "Tmux utility for session and window management";
            homepage = "https://github.com/EdenEast/tuxmux";
            license = with licenses; [ mit ];
            mainProgram = "tm";
          };
        });

      in
      rec
      {
        checks = {
          clippy = craneLib.cargoClippy (args // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
            doCheck = true;
          });
          tests = craneLib.cargoTest (args // {
            inherit cargoArtifacts;
            doCheck = true;
          });

        };

        apps = {
          tuxmux = flake-utils.lib.mkApp {
            dev = tuxmux;
            name = "tm";
          };
          default = apps.tuxmux;
        };

        packages = {
          inherit tuxmux;
          default = tuxmux;
        };

        devShells.default = pkgs.mkShell {
          name = "tuxmux";
          inputsFrom = builtins.attrValues checks;
          nativeBuildInputs = with pkgs; [
            rustToolchain
          ];
          packages = with pkgs; [
            just
          ];
          # RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library";
        };
      });
}
