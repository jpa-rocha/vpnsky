{ pkgs, cargo, ... }:
{
  lint = pkgs.writeShellScriptBin "lint" ''
    set -euo pipefail
    ${pkgs.nix}/bin/nix flake check
  '';

  build = pkgs.writeShellScriptBin "build" ''
    set -euo pipefail
    ${cargo}/bin/sh build -r
  '';

}
