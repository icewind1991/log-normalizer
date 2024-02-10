{
  stdenv,
  rustPlatform,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
  src = sourceByRegex ./. ["Cargo.*" "(src|tests|sqlx-data.json)(/.*)?"];
in
  rustPlatform.buildRustPackage rec {
    pname = "log-normalizer";
    version = "0.1.0";

    SQLX_OFFLINE = 1;

    inherit src;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };
  }
