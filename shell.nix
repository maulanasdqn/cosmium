# Cosmium Chromium build environment for NixOS / nix-shell.
#
# Usage:
#   nix-shell              # enters the shell with all deps
#   ./scripts/build-linux.sh   # fetch + patch + compile
#
# Tested on NixOS 24.11+. If a package name changes across channels,
# override via `nixpkgs` pin or `--arg pkgs '...'`.
{ pkgs ? import <nixpkgs> {} }:

let
  # Libraries that host-side build tools (wayland_scanner, protoc-alikes,
  # mojo/blink generators) link against. use_sysroot only governs the *target*
  # toolchain; host tools link the Nix libs above and are emitted without an
  # rpath, so they need an explicit LD_LIBRARY_PATH to run during the build.
  hostToolLibs = with pkgs; [
    expat glib nss nspr zlib bzip2 icu libpng freetype fontconfig
    dbus atk at-spi2-atk at-spi2-core cairo pango gtk3 alsa-lib
    libxkbcommon libdrm mesa cups flac harfbuzz libva libglvnd
    stdenv.cc.cc.lib
  ];
in

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

    # Chromium compiles against its own pinned sysroot (use_sysroot = true in
    # config/args.gn), so system library headers must NOT leak in. Nix points
    # PKG_CONFIG_PATH at nix-store .pc files; Chromium's
    # build/config/linux/pkg-config.py would then resolve glib/gtk/nss from
    # the store and blindly prefix the sysroot onto those absolute paths,
    # emitting include dirs like
    #   build/linux/debian_bullseye_amd64-sysroot/nix/store/…/include/glib-2.0
    # which cannot exist — the build dies on "'glib.h' file not found".
    # Clearing these lets the sysroot's own .pc files resolve.
    unset PKG_CONFIG_PATH
    unset PKG_CONFIG_LIBDIR

    # Host build tools are linked against the Nix libs without an rpath, so
    # ninja actions that execute them fail at runtime, e.g.
    #   ./wayland_scanner: error while loading shared libraries:
    #     libexpat.so.1: cannot open shared object file
    # (exit 127, surfacing as a failed wayland_scanner_wrapper.py action).
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath hostToolLibs}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
    echo "🛠  cosmium build shell ready ($(nproc) cores, $(free -g | awk '/Mem/{print $2}')G RAM)"
    echo "   run: ./scripts/build-linux.sh"
  '';
}
