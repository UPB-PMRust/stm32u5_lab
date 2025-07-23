let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
  PROJECT_ROOT = builtins.getEnv "PWD";
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    clang
    llvmPackages.bintools
    rustup
    cargo-binutils
    probe-rs

    # Language Server
    taplo
  ];
}

