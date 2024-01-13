{
  darwin,
  lib,
  libpqxx,
  nix-gitignore,
  rustPlatform,
  stdenv,
}: let
  name = "capture-dn";
  path = ../.;
  inherit (stdenv) isDarwin;
  inherit (darwin.apple_sdk_11_0.frameworks) CoreFoundation Security SystemConfiguration;
in
  rustPlatform.buildRustPackage {
    inherit name;
    src = nix-gitignore.gitignoreSource [] (builtins.path {inherit name path;});
    cargoLock.lockFile = ../Cargo.lock;
    buildInputs =
      [libpqxx]
      ++ lib.optionals isDarwin [
        CoreFoundation
        Security
        SystemConfiguration
      ];
  }
