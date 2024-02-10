{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.services.log-normalizer;
in {
  options.services.log-normalizer = {
    enable = mkEnableOption "Log normalizer";

    user = mkOption {
      type = types.str;
      description = "user to run as";
    };

    databaseUrlFile = mkOption {
      type = types.str;
      description = "file containg DATABASE_URL variable";
    };

    rawDatabaseUrlFile = mkOption {
      type = types.str;
      description = "file containg RAW_DATABASE_URL variable";
    };

    logLevel = mkOption {
      type = types.str;
      default = "info,sqlx=warn";
      description = "log level";
    };

    package = mkOption {
      type = types.package;
      description = "package to use";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.log-normalizer = {
      wantedBy = ["multi-user.target"];
      environment = {
        RUST_LOG = cfg.logLevel;
      };

      serviceConfig = {
        EnvironmentFile = [cfg.databaseUrlFile cfg.rawDatabaseUrlFile];
        ExecStart = "${cfg.package}/bin/log-normalizer";
        Restart = "on-failure";
        User = cfg.user;
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        NoNewPrivileges = true;
        PrivateDevices = true;
        ProtectClock = true;
        CapabilityBoundingSet = true;
        ProtectKernelLogs = true;
        ProtectControlGroups = true;
        SystemCallArchitectures = "native";
        ProtectKernelModules = true;
        RestrictNamespaces = true;
        MemoryDenyWriteExecute = true;
        ProtectHostname = true;
        LockPersonality = true;
        ProtectKernelTunables = true;
        RestrictAddressFamilies = "AF_INET AF_INET6 AF_UNIX";
        RestrictRealtime = true;
        ProtectProc = "noaccess";
        SystemCallFilter = ["@system-service" "~@resources" "~@privileged"];
        IPAddressDeny = "any";
        IPAddressAllow = "localhost";
        PrivateUsers = true;
        ProcSubset = "pid";
      };
    };
  };
}
