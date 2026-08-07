# Cosmium Chromium build environment for NixOS / nix-shell.
#
# Usage:
#   nix-shell              # enters the shell with all deps
#   ./scripts/build-linux.sh   # fetch + patch + compile
#
# Tested on NixOS 24.11+. If a package name changes across channels,
# override via `nixpkgs` pin or `--arg pkgs '...'`.
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "cosmium-build";

  nativeBuildInputs = with pkgs; [
    # Core build tools
    python3
    ninja
    gn
    git
    curl
    pkg-config
    which
    perl

    # Compilers — Chromium ships its own clang, but system clang is
    # needed during bootstrap (cipd, gn, etc.)
    clang
    lld
    llvmPackages.bintools

    # Required by install-build-deps / gn gen
    glib
    nss
    nspr
    atk
    at-spi2-atk
    at-spi2-core
    cups
    dbus
    libdrm
    mesa
    pango
    cairo
    gtk3
    alsa-lib
    libxkbcommon
    libpulseaudio
    systemd          # libudev
    expat
    flac
    libpng
    zlib
    bzip2
    icu
    harfbuzz
    freetype
    fontconfig

    # X11
    xorg.libX11
    xorg.libXcomposite
    xorg.libXcursor
    xorg.libXdamage
    xorg.libXext
    xorg.libXfixes
    xorg.libXi
    xorg.libXrandr
    xorg.libXrender
    xorg.libXScrnSaver
    xorg.libXtst
    xorg.libxcb
    xorg.libxshmfence

    # Wayland
    wayland
    wayland-protocols

    # Misc
    pciutils
    libva
    libglvnd
  ];

  shellHook = ''
    export CHROMIUM_BUILDTOOLS_PATH="$PWD/src/buildtools"
    export PATH="$PWD/depot_tools:$PATH"
    echo "🛠  cosmium build shell ready ($(nproc) cores, $(free -g | awk '/Mem/{print $2}')G RAM)"
    echo "   run: ./scripts/build-linux.sh"
  '';
}
