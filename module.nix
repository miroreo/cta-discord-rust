{
  config,
  pkgs,
  lib,
  ...
}: let
  default_db_url = "postgres://localhost/cta-discord?host=/run/postgresql";
  path2derivation = path: pkgs.runCommand (builtins.toString path) {} ''cp -r ${path} $out'';
  inherit (lib) mkEnableOption mkPackageOption mkIf mkOption types optionalAttrs;
in {
  options.services.cta-discord = {
    enable = mkEnableOption "cta-discord";
    package = mkPackageOption pkgs "cta-discord" {};

    db-url = mkOption {
      type = types.str;
      default = default_db_url;
      description = ''
        URL of CTA Discord's database.
      '';
    };

    assetsDir = mkOption {
      type = types.str;
      default = builtins.toString (path2derivation ./assets);
      description = ''
        The directory where the CTA Bot reads its assets.
      '';
    };

    args = mkOption {
      type = types.listOf types.str;
      default = [];
    };

    envFile = mkOption {
      type = types.str;
      default = builtins.toString (pkgs.writeText "default.env" "");
    };

    user = mkOption {
      type = types.str;
      default = "cta-discord";
      description = "User account under which CTA Discord runs.";
    };

    group = mkOption {
      type = types.str;
      default = "cta-discord";
      description = "Group account under which CTA Discord runs.";
    };

    development = mkOption {
      type = types.bool;
      default = false;
      description = "Enables development features for CTA Discord.";
    };
  };

  config = let
    cfg = config.services.cta-discord;
    cta-discord = cfg.package + "/bin/cta-discord";
  in
    mkIf cfg.enable {
      systemd.services.cta-discord = {
        description = "CTA Discord Bot";
        after = ["network.target"];
        wantedBy = ["multi-user.target"];

        environment = {
          DATABASE_URL = "${cfg.db-url}";
          DEVELOPMENT = mkIf (cfg.development) "1";
          ASSETS_DIR = "${cfg.assetsDir}";
        };

        serviceConfig = {
          Type = "simple";
          User = cfg.user;
          Group = cfg.group;

          EnvironmentFile = cfg.envFile;
          CacheDirectory = "cta-discord";
          ExecStart = pkgs.writeScript "cta-discord-start" ''
            #!/bin/sh
            ${cta-discord} ${builtins.concatStringsSep " " cfg.args}
          '';
          Restart = "always";
          PrivateTmp = true;
          ProtectHome = "tmpfs";
        };
      };

      users.users = optionalAttrs (cfg.user == "cta-discord") {
        cta-discord = {
          group = cfg.group;
          isSystemUser = true;
        };
      };

      users.groups = optionalAttrs (cfg.group == "cta-discord") {
        cta-discord.members = [cfg.user];
      };

      services.postgresql = mkIf (cfg.db-url == default_db_url) {
        enable = true;
        ensureUsers = [
          {
            name = "${cfg.user}";
            ensureDBOwnership = true;
          }
        ];
        ensureDatabases = ["cta-discord"];
      };
    };
}
