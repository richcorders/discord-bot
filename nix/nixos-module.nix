{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.discord-richbot;
  databaseConfig = {
    services.postgresql = {
      enable = true;
      ensureDatabases = lib.singleton cfg.settings.db.name;
      ensureUsers = lib.singleton {
        name = cfg.settings.db.user;
        ensureDBOwnership = true;
      };
      authentication = ''
        host ${cfg.settings.db.name} ${cfg.settings.db.user} 127.0.0.1/32 trust
      '';
    };

    systemd.services.discord-richbot = {
      requires = [ "postgresql.service" ];
      after = [ "postgresql.service" ];
    };
  };

  inherit (lib) types;
in
{
  options.services.discord-richbot = {
    enable = lib.mkEnableOption "discord-richbot";
    package = lib.mkPackageOption pkgs "discord-richbot";

    db = {
      host = lib.mkOption {
        type = types.str;
        default = "127.0.0.1";
        description = ''
          The database host to use for Richbot.
        '';
      };
      port = lib.mkOption {
        type = types.int;
        default = 5432;
        description = ''
          The port to use for the database.
        '';
      };
      name = lib.mkOption {
        type = types.str;
        default = "richbot";
        description = ''
          The database name to use for Richbot;
        '';
      };
      user = lib.mkOption {
        type = types.str;
        default = "richbot";
        description = ''
          The database user to use for Richbot
        '';
      };
      password = lib.mkOption {
        type = types.nullOr types.str;
        default = null;
        description = ''
          The database password to use for Richbot
        '';
      };
    };
  };
}
