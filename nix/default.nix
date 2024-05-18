{
  darwin,
  lib,
  postgresql,
  nix-gitignore,
  rustPlatform,
  stdenv,
}:
let
  name = "richbot";
  version = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).package.version;
  path = ../.;
  inherit (stdenv) isDarwin;
  inherit (darwin.apple_sdk_11_0.frameworks) CoreFoundation Security SystemConfiguration;
in
rustPlatform.buildRustPackage {
  inherit name version;
  src = nix-gitignore.gitignoreSource [ ] (builtins.path { inherit name path; });
  cargoLock.lockFile = ../Cargo.lock;
  buildInputs =
    [ postgresql ]
    ++ lib.optionals isDarwin [
      CoreFoundation
      Security
      SystemConfiguration
    ];
}
