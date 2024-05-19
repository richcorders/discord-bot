# SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
# SPDX-License-Identifier: CC0-1.0

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devenv.url = "github:cachix/devenv";
    git-hooks.follows = "devenv/pre-commit-hooks";
    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
        inputs.git-hooks.flakeModule
      ];
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      perSystem =
        {
          config,
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
            enterShell = config.pre-commit.devShell.shellHook;
            process.implementation = "overmind";

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

            services.postgres = {
              enable = true;
              initialDatabases = [ { name = "dn"; } ];
              listen_addresses = "127.0.0.1";
            };
          };

          pre-commit = {
            check.enable = false;
            settings.hooks = {
              clippy.enable = true;
              commitizen.enable = true;
              nixfmt = {
                enable = true;
                package = pkgs.nixfmt-rfc-style;
              };
              reuse = {
                enable = true;
                name = "reuse";
                entry = "${pkgs.reuse}/bin/reuse lint";
                pass_filenames = false;
              };
              rustfmt = {
                enable = true;
                packageOverrides.rustfmt = pkgs.fenix.latest.rustfmt;
              };
            };
          };

          packages.default = pkgs.callPackage ../default.nix { };
        };
    };

  nixConfig = {
    extra-substituters = [
      "https://devenv.cachix.org"
      "https://pre-commit-hooks.cachix.org"
      "https://nix-community.cachix.org/"
    ];
    extra-trusted-public-keys = [
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "pre-commit-hooks.cachix.org-1:Pkk3Panw5AW24TOv6kz3PvLhlH8puAsJTBbOPmBo7Rc="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}