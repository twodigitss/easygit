{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "easygit";
  version = "0.1.0";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  installPhase = ''
    runHook preInstall
    install -Dm755 target/release/easygit $out/bin/easygit
    runHook postInstall
  '';

  meta = with pkgs.lib; {
    description = "A simple git helper written in Rust";
    homepage = "https://github.com/twodigitss/easygit";
    license = licenses.mit;
    maintainers = with maintainers; [ "twodigitss" ];
    platforms = platforms.all;
  };
}
