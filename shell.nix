let
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [ (import rustOverlay) ];
  };
in
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    # Essentials for PyTorch, etc.
    zlib
    stdenv.cc.cc.lib
    openssl
    libtorch-bin

    # Compiler Chain [optional]
    mold
    clang
    pkg-config
    gettext


    # Language Setup
    python311
    python311Packages.pip
    (rust-bin.stable."1.67.0".default.override {
      extensions = [ "rust-src" "clippy" ];
    })

    # Dev Tooling
    rust-analyzer
    cargo-edit
    docker-compose
    linuxPackages.perf
  ];

  RUST_BACKTRACE = 1;
  MOLD_PATH = "${pkgs.mold.out}/bin/mold";
  RUSTFLAGS = "-Clink-arg=-fuse-ld=${MOLD_PATH} -Clinker=clang";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
