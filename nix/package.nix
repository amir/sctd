{
  lib,
  rustPlatform,
  pkg-config,
  xorg,
  src,
}:

let
  cargoToml = lib.importTOML "${src}/Cargo.toml";
in
rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  inherit src;

  cargoLock.lockFile = "${src}/Cargo.lock";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [
    xorg.libX11
    xorg.libXrandr
  ];

  meta = with lib; {
    description = "Set color temperature daemon (X11)";
    longDescription = ''
      sctd calculates sunrise and sunset based on a given latitude and
      longitude and smoothly adjusts the screen color temperature
      accordingly. Based on sct by Ted Unangst, with transition logic
      inspired by Redshift.
    '';
    homepage = "https://github.com/amir/sctd";
    license = licenses.cc0;
    mainProgram = "sctd";
    platforms = platforms.linux;
  };
}
