# SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
# SPDX-License-Identifier: CC0-1.0
{
  config,
  inputs,
  lib,
  pkgs,
  ...
}:
let
  inherit (pkgs) stdenv;
  fenixPkgs = inputs.fenix.packages.${stdenv.system};
in

{
  process.implementation = "overmind";

  dotenv.enable = true;

  # https://devenv.sh/packages/
  packages = [
    pkgs.diesel-cli
    pkgs.nixfmt-rfc-style
    pkgs.reuse
  ];

  # https://devenv.sh/scripts/
  # https://devenv.sh/tests/

  # https://devenv.sh/services/
  services.postgres = {
    enable = true;
    initialDatabases = [ { name = "richbot"; } ];
    listen_addresses = "127.0.0.1";
  };

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      toolchain.rustfmt = fenixPkgs.latest.rustfmt;
    };
  };

  # https://devenv.sh/pre-commit-hooks/
  pre-commit.hooks = {
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
      packageOverrides.rustfmt = fenixPkgs.latest.rustfmt;
    };
  };

  # https://devenv.sh/processes/

  # See full reference at https://devenv.sh/reference/options/
}
