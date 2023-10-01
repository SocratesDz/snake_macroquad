{ pkgs ? import <nixpkgs> {} }:

with pkgs;
mkShell rec {
  buildInputs = [
    rustup
    pkg-config

    libxkbcommon
    libGL

    # WINIT_UNIX_BACKEND=wayland
    wayland

    # WINIT_UNIX_BACKEND=x11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11

    udev
    alsaLib
    xorg.libXi
    xorg.libXrandr
  ];
  LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
}
