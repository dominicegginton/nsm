{pkgs}:
with pkgs.rustPlatform;
with pkgs.lib.sources;
with builtins; let
  cargoToml = fromTOML (readFile ./Cargo.toml);
in
  buildRustPackage {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;
    src = cleanSource ./.;
    cargoSha256 = "sha256-BV1dHcWly4OREPGioafNt/OQVnNMiZWIsLzqI/85esg=";
    nativeBuildInputs = with pkgs; [
      rustc
      cargo
    ];
  }
