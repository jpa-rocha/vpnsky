{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.vpnsky;
  inherit (lib)
    mkEnableOption
    mkIf
    mkOption
    types
    ;
in
{
  options.programs.vpnsky = {
    enable = mkEnableOption "vpnsky VPN client";

    package = mkOption {
      type = types.package;
      description = "The vpnsky package to use.";
    };

    settings = {
      address = mkOption {
        type = types.str;
        description = "VPN gateway address.";
        example = "vpn.example.com";
      };

      mtu = mkOption {
        type = types.int;
        default = 1320;
        description = "MTU for the VPN tunnel interface.";
      };
    };

    logs = {
      toFile = mkOption {
        type = types.bool;
        default = false;
        description = "Whether to write logs to a file.";
      };

      path = mkOption {
        type = types.str;
        default = "./logs";
        description = "Directory path for log files (only used when toFile is true).";
      };

      level = mkOption {
        type = types.enum [
          "DEBUG"
          "INFO"
          "WARN"
          "ERROR"
        ];
        default = "INFO";
        description = "Log verbosity level.";
      };
    };

    secrets = {
      path = mkOption {
        type = types.str;
        description = "Path to the sops-encrypted secrets YAML file.";
        example = "/home/user/nixos/secrets.yaml";
      };
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile."vpnsky.toml".source = (pkgs.formats.toml { }).generate "vpnsky.toml" {
      logs = {
        to_file = cfg.logs.toFile;
        path = cfg.logs.path;
        level = cfg.logs.level;
      };
      settings = {
        address = cfg.settings.address;
        mtu = cfg.settings.mtu;
      };
      secrets = {
        path = cfg.secrets.path;
      };
    };
  };
}
