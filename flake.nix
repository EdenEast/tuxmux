{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        toolchainManifest = builtins.fromTOML (builtins.readFile ./rust-toolchain.toml);
        version = manifest.package.version;
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # inherit (pkgs) lib;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common configuration needed for crane to build the rust project
        args = {
          src = ./.;

          # This is not required as this would just compile the project again
          doCheck = false;
          buildInputs = with pkgs; [
            libiconv
          ];
        };

        # Build *just* the cargo dependencies, so we can reuse all of that work between runs
        # This also makes sure that the `build.rs` file is built. If buildPackage is just called
        # the build.rs file was not being executed.
        cargoArtifacts = craneLib.buildDepsOnly args;

        tuxmux = craneLib.buildPackage (args // {
          inherit cargoArtifacts version;

          doCheck = true;

          nativeBuildInputs = with pkgs; [
            # Needed for installing shell completions and manpages
            installShellFiles
          ];

          preFixup = ''
            installManPage target/man/*
            installShellCompletion --bash target/completions/tux.bash
            installShellCompletion --zsh target/completions/_tux
            installShellCompletion --fish target/completions/tux.fish
          '';

          meta = with pkgs.lib; {
            description = "Tmux utility for session and window management";
            homepage = "https://github.com/EdenEast/tuxmux";
            license = licenses.apsl20;
            mainProgram = "tux";
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
            drv = tuxmux;
            name = "tux";
          };
          default = apps.tuxmux;
        };

        packages = {
          inherit tuxmux;
          default = tuxmux;
        };

        devShells.default =
          let
            rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
              extensions = toolchainManifest.toolchain.components ++ [ "rust-analyzer" ];
            };
          in
          pkgs.mkShell {
            name = "tuxmux";
            inputsFrom = builtins.attrValues checks;
            nativeBuildInputs = with pkgs; [
              rust
              cargo-deny
            ];
            packages = with pkgs; [
              asciidoctor-with-extensions
              jq
              just
              pandoc
            ];
          };
      });
}
