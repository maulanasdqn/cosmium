{ pkgs ? import <nixpkgs> {} }:

let
  gperf31 = pkgs.gperf.overrideAttrs (old: rec {
    version = "3.1";
    src = pkgs.fetchurl {
      url = "mirror://gnu/gperf/gperf-${version}.tar.gz";
      sha256 = "1qispg6i508rq8pkajh26cznwimbnj06wq9sd85vg95v8nwld1aq";
    };
  });

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
    python3
    ninja
    gn
    git
    curl
    pkg-config
    which
    perl

    gperf31
    bison
    flex

    clang
    lld
    llvmPackages.bintools

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
    systemd
    expat
    flac
    libpng
    zlib
    bzip2
    icu
    harfbuzz
    freetype
    fontconfig

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

    wayland
    wayland-protocols

    pciutils
    libva
    libglvnd
  ];

  shellHook = ''
    export CHROMIUM_BUILDTOOLS_PATH="$PWD/src/buildtools"
    export PATH="$PWD/depot_tools:$PATH"

    unset PKG_CONFIG_PATH
    unset PKG_CONFIG_LIBDIR

    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath hostToolLibs}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
    echo "🛠  cosmium build shell ready ($(nproc) cores, $(free -g | awk '/Mem/{print $2}')G RAM)"
    echo "   run: ./scripts/build-linux.sh"
  '';
}
