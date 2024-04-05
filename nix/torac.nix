{ lib, cargo-tauri, rustPlatform, mkYarnPackage, fetchYarnDeps, curl, dbus, glib
, gtk3, librsvg, libsoup, openssl_3, wget, pkg-config, webkitgtk }:

let
  pname = "torac";
  version = "0.1.0";
  src = ../.;

  frontend-package = mkYarnPackage {
    inherit version src;
    pname = pname + "-front";

    offlineCache = fetchYarnDeps {
      yarnLock = "${src}/yarn.lock";
      hash = "sha256-C8/yExuDV9GnhoWm8QuVp+RR4gdDDsKAXqV8qkfotcQ=";
    };

    packageJSON = ../package.json;

    configurePhase = ''
      runHook preConfigure
      ln -s $node_modules node_modules
      runHook postConfigure
    '';

    buildPhase = ''
      runHook preBuild

      export HOME=$(mktemp -d)
      yarn --offline build

      runHook postBuild
    '';

    postBuild = ''
      mkdir -p $out/dist
      cp -r dist/** $out/dist
    '';

    distPhase = "true";
    dontInstall = true;
  };
in rustPlatform.buildRustPackage {
  inherit version pname;
  src = ../src-tauri;

  cargoLock = { lockFile = ../src-tauri/Cargo.lock; };

  postPatch = ''
    mkdir -p dist
    cp -R ${frontend-package}/dist/** dist

    substituteInPlace ./tauri.conf.json \
      --replace-fail '"distDir": "../dist"' '"distDir": "./dist"' \
      --replace-fail '"beforeBuildCommand": "yarn build",' '"beforeBuildCommand": "",'
  '';

  nativeBuildInputs = [ pkg-config cargo-tauri ];

  buildInputs =
    [ curl dbus glib gtk3 librsvg libsoup openssl_3 wget webkitgtk ];

  buildPhase = ''
    runHook preBuild

    export VERGEN_GIT_DESCRIBE=${version}
    export RUST_BACKTRACE=1
    cargo tauri build

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin/
    mkdir -p $out/share/

    cp target/release/bundle/deb/torac_0.0.0_amd64/data/usr/bin/torac $out/bin/torac/
    cp -R target/release/bundle/deb/torac_0.0.0_amd64/data/usr/torac/** $out/share/

    runHook postInstall
  '';

  postFixup = ''
    wrapProgram "$out/bin/torac" \
      --set WEBKIT_DISABLE_COMPOSITING_MODE 1
  '';

  meta = with lib; {
    description = "Placeholder";
    homepage = "https://github.com/Torac-d-o-o/torac-front";
    license = licenses.mpl20;
    mainProgram = "torac";
    platforms = [ "x86_64-linux" "x86_64-windows" ];
    maintainers = [{
      name = "wizardlink";
      email = "contact@thewizard.link";
      github = "wizardlink";
      githubId = 26727907;
      keys = [{
        fingerprint = "A1D3 A2B4 E14B D7C0 445B  B749 A576 7B54 367C FBDF";
      }];
    }];
  };
}
