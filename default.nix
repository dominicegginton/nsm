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
    cargoSha256 = "sha256-g73aagINMNM2ARN7BBejulOWwfGjkKvkUlcflulzPo0=";
    nativeBuildInputs = with pkgs; [
      rustc
      cargo
    ];
  }
