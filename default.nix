{
  pkgs ? import <nixpkgs> {},
  rustPlatform ? pkgs.rustPlatform,
}:
with rustPlatform;
  buildRustPackage rec {
    name = "nsm";
    pname = "nsm";
    version = "0.1.0";
    src = ./.;
    cargoSha256 = "sha256-9mbA7yTzCPEbDUobS2cdSI9sZApBaPSJQqWgFlzgXHU=";
    nativeBuildInputs = with pkgs; [
      rustc
      cargo
    ];
  }
