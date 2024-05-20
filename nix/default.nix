{ rustPlatform
, pkg-config
, wrapGAppsHook4
, gtk4
, libadwaita
, lib
, lockFile
, cargo
, rustc
, ...
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
rustPlatform.buildRustPackage rec {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  src = ../.;

  buildInputs = [
    pkg-config
    gtk4
    libadwaita
  ];

  cargoLock = {
    inherit lockFile;
  };

  checkInputs = [ cargo rustc ];

  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook4
    cargo
    rustc
  ];
  copyLibs = true;

  postInstall = ''
    	install -D --mode=444 $src/${pname}.desktop $out/share/applications/${pname}.desktop
    	install -D --mode=444 $src/${pname}.svg $out/share/pixmaps/${pname}.svg
  '';

  meta = with lib; {
    description = "A small, simple calculator written in rust/gtk4";
    homepage = "https://github.com/DashieTM/OxiCalc";
    changelog = "https://github.com/DashieTM/OxiCalc/releases/tag/${version}";
    license = licenses.gpl3;
    maintainers = with maintainers; [ DashieTM ];
    mainProgram = "oxicalc";
  };
}
