{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, utils, ... }@inputs:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = with inputs; [ fenix.overlays.default ];
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

        fenixRustPackages = (with pkgs.fenix; with stable; combine [
          cargo
          clippy
          rust-analyzer
          rust-src
          rustc
          rustfmt
          targets.x86_64-pc-windows-msvc.stable.rust-std
        ]);

        torac = pkgs.callPackage ./nix/torac.nix { };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              curl
              dbus
              fenixRustPackages
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
