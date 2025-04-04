{
  inputs,
  pkgs,
}:
pkgs.mkShellNoCC {
  packages = [
    inputs.flake-ci.packages.${pkgs.system}.default
  ];
}
