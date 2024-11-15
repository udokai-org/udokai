{
  pkgs ? import <nixpkgs> {}
}:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    libiconv
    xdotool

    dioxus-cli

    # dioxux deps
    # openssl
    pkg-config
    libusb
    libusb1
    libusb1.dev
    cairo
    libsoup_3
    webkitgtk_4_1
  ];
}
