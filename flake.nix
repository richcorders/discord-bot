# SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
# SPDX-License-Identifier: CC0-1.0

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devenv.url = "github:cachix/devenv";
    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          lib,
          pkgs,
          self',
          system,
          ...
        }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.fenix.overlays.default ];
          };

          formatter = pkgs.nixfmt-rfc-style;

          devenv.shells.default = {
            containers = lib.mkForce { };
            languages.rust = {
              enable = true;
              toolchain.rustfmt = pkgs.fenix.latest.rustfmt;
            };
            packages =
              self'.packages.default.buildInputs
              ++ [ pkgs.diesel-cli ]
              ++ [
                self'.formatter
                pkgs.reuse
              ];
            pre-commit.hooks = {
              clippy.enable = true;
              nixfmt.enable = true;
              nixfmt.package = pkgs.nixfmt-rfc-style;
              reuse = {
                enable = true;
                name = "REUSE Compliance Check";
                entry = "${pkgs.reuse}/bin/reuse lint";
                pass_filenames = false;
              };
              rustfmt.enable = true;
            };
            services.postgres = {
              enable = true;
              initialDatabases = [ { name = "dn"; } ];
              listen_addresses = "127.0.0.1";
            };
          };

          packages.default = pkgs.callPackage ./nix { };
        };
    };

  nixConfig = {
    extra-substituters = [
      "https://nekowinston.cachix.org"
      "https://nix-community.cachix.org/"
    ];
    extra-trusted-public-keys = [
      "nekowinston.cachix.org-1:lucpmaO+JwtoZj16HCO1p1fOv68s/RL1gumpVzRHRDs="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}
