{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    pre-commit-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.flake-compat.follows = "flake-compat";
      inputs.flake-utils.follows = "flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:cachix/devenv/python-rewrite";
      inputs.flake-compat.follows = "flake-compat";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.pre-commit-hooks.follows = "pre-commit-nix";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # debloat
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-compat.flake = false;
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [inputs.devenv.flakeModule];
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        pkgs,
        self',
        system,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.fenix.overlays.default];
        };
        formatter = pkgs.alejandra;
        devenv.shells.default = {
          containers = pkgs.lib.mkForce {};
          dotenv.enable = true;
          languages.rust = {
            enable = true;
            toolchain.rustfmt = pkgs.fenix.latest.rustfmt;
          };
          packages = [pkgs.diesel-cli] ++ self'.packages.default.buildInputs;
          pre-commit.hooks = {
            alejandra.enable = true;
            clippy.enable = true;
            rustfmt.enable = true;
          };
          services.postgres = {
            enable = true;
            initialDatabases = [{name = "dn";}];
            listen_addresses = "127.0.0.1";
          };
        };
        packages.default = pkgs.callPackage ./nix {};
      };
    };

  # for the fenix overlay
  nixConfig = {
    extra-substituters = ["https://nix-community.cachix.org/"];
    extra-trusted-public-keys = ["nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="];
  };
}
