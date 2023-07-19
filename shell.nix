# <shell.nix>
{ pkgs ? import <nixpkgs> {}}:

let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  ruststable = (pkgs.rust-bin.stable.latest.default.override {
    extensions = [
      "rust-src"
    ];
  });
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    ruststable
    rust-analyzer
    bacon
    commitizen
    #pkg-config
  ];
}

