{pkgs, ...}:
with pkgs;
  mkShell rec {
    nativeBuildInputs = with pkgs; [
      rustc
      cargo
      rust-analyzer
      rustfmt
    ];
  }
