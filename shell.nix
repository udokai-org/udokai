{
  pkgs ? import <nixpkgs> {}
}:
let
  libPath = with pkgs; lib.makeLibraryPath [
    libGL
    libxkbcommon
    wayland
  ];
in pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    libiconv
  ] ++ lib.optionals pkgs.stdenv.isLinux (with pkgs; [
    # Linux deps
  ]) ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
    # MacOs deps
  ]);

  RUST_LOG = "debug";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = libPath;
}
