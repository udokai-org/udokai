{
  pkgs ? import <nixpkgs> {}
}:
pkgs.mkShell {
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
    libusb
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
}
