let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  unstable-nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-unstable";

  pkgs = import nixpkgs { config = {}; overlays = []; };
  unstable-pkgs = import unstable-nixpkgs { config = {}; overlays = []; };

  rust-packages = with pkgs; [
    rustc
    cargo

    rustfmt
    rust-analyzer
  ];

  unstable-packages = with unstable-pkgs; [
    maelstrom-clj
  ];
in

pkgs.mkShellNoCC {
  packages = rust-packages ++ unstable-packages;
}
