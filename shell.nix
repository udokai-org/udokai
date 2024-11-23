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

    dioxus-cli
    # dioxux deps
    # openssl
  ] ++ lib.optionals pkgs.stdenv.isLinux (with pkgs; [
    pkg-config
    libusb1
    libusb1.dev
    cairo
    libsoup_3
    webkitgtk_4_1
    xdotool
  ]) ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
    IOKit
    Carbon
    WebKit
    Security
    Cocoa
  ]);

  RUST_LOG = "debug";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = libPath;

  # shellHook = ''
  #   ${pkgs.cargo}/bin/cargo update
  # '';
}
