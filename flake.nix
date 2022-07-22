{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec {
        # `nix build`
        packages.log-normalizer = naersk-lib.buildPackage {
          pname = "log-normalizer";
          root = ./.;

          SQLX_OFFLINE = 1;
        };
        defaultPackage = packages.log-normalizer;
        defaultApp = packages.log-normalizer;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustc cargo bacon cargo-edit cargo-outdated];
        };
      }
    )
    // {
      nixosModule = {
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
          };

          config = mkIf cfg.enable {
            systemd.services."log-normalizer" = let
              pkg = self.defaultPackage.${pkgs.system};
            in {
              wantedBy = ["multi-user.target"];
              script = "${pkg}/bin/log-normalizer";
              environment = {
                RUST_LOG = cfg.logLevel;
              };

              serviceConfig = {
                EnvironmentFile = [cfg.databaseUrlFile cfg.rawDatabaseUrlFile];
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
        };
    };
}
