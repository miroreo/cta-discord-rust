{
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib) mkEnableOption mkPackageOption mkIf mkOption types optionalAttrs;
in {
  options.services.cta-discord-rust = {
    enable = mkEnableOption "cta-discord-rust";
    package = mkPackageOption pkgs "cta-discord-rust";
  };

  config = let
    cfg = config.services.cta-discord-rust;
  in
    mkIf cfg.enable {
    };
}
