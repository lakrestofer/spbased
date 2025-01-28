# This flake was initially generated by fh, the CLI for FlakeHub (version 0.1.21)
{
  # A helpful description of your flake
  description = "content agnostic spaced repetition";

  # Flake inputs
  inputs = {
    flake-schemas.url = "https://flakehub.com/f/DeterminateSystems/flake-schemas/*";

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-root.url = "github:srid/flake-root";
  };

  # Flake outputs that other flakes can use
  outputs =
    {
      self,
      flake-schemas,
      nixpkgs,
      rust-overlay,
      flake-parts,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs self; } {
      imports = [
        inputs.flake-root.flakeModule
        inputs.flake-parts.flakeModules.easyOverlay
      ];
      flake = {
        # Schemas tell Nix about the structure of your flake's outputs
        schemas = flake-schemas.schemas;
      };
      systems = [
        "x86_64-linux"
      ];
      perSystem =
        {
          pkgs,
          self',
          system,
          config,
          ...
        }:
        let
          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
            rustc = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
          };
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
              (final: prev: {
                rustToolchain = final.rust-bin.selectLatestNightlyWith (
                  toolchain:
                  toolchain.default.override {
                    extensions = [ "rust-src" ];
                  }
                );
              })
            ];
            config = { };
          };
          flake-root.projectRootFile = "flake.nix"; # Not necessary, as flake.nix is the default
          devShells.default = pkgs.mkShell {
            inputsFrom = [ config.flake-root.devShell ]; # Provides $FLAKE_ROOT in dev shell
            packages = with pkgs; [
              glow
              nodejs
              gum
              jq
              rustToolchain
              cargo-bloat
              cargo-edit
              rust-analyzer
            ];
            env = {
              # RUST_BACKTRACE = "1";
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
          packages =
            let
              script_dependencies = with pkgs; [
                glow
                gum
                jq
              ];
              create_script =
                { name, src }:
                let
                  script = (pkgs.writeScriptBin name src).overrideAttrs (old: {
                    buildCommand = "${old.buildCommand}\n patchShebangs $out";
                  });
                in
                pkgs.symlinkJoin {
                  inherit name;
                  paths = [ script ] ++ script_dependencies;
                  postBuild = "wrapProgram $out/bin/${name} --prefix PATH : $out/bin";
                  buildInputs = [ pkgs.makeWrapper ];
                };
            in
            {
              default = self'.packages.spbasedctl;
              spbasedctl = rustPlatform.buildRustPackage {
                inherit (cargoToml) version;
                name = "spbasedctl";
                src = ./.;
                cargoLock.lockFile = ./Cargo.lock;
                cargoBuildFlags = "--package spbasedctl";
              };
              flashcard = create_script {
                name = "flashcard";
                src = builtins.readFile ./scripts/flashcard;
              };
            };
        };
    };
}
