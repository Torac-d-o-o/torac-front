{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";

    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, utils, ... }@inputs:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = with inputs; [ (import nixpkgs-mozilla) ];
        };

        libraries = with pkgs; [
          cairo
          dbus
          gdk-pixbuf
          glib
          gtk3
          librsvg
          openssl_3
          webkitgtk
        ];

        toolchain = (pkgs.rustChannelOf {
          rustToolchain = ./src-tauri/rust-toolchain.toml;
          sha256 = "sha256-opUgs6ckUQCyDxcB9Wy51pqhd0MPGHUVbwRKKPGiwZU=";
        }).rust;

        torac = pkgs.callPackage ./nix/torac.nix { };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              curl
              dbus
              glib
              gtk3
              librsvg
              libsoup
              lld
              llvm
              nodejs
              nsis
              openssl_3
              pkg-config
              toolchain
              wget
              yarn
            ] ++ libraries;

          shellHook = ''
            export LD_LIBRARY_PATH=${
              pkgs.lib.makeLibraryPath libraries
            }:$LD_LIBRARY_PATH
            export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
          '';
        };

        packages.default = torac;

        formatter.${system} = pkgs.nixfmt;
      });
}
