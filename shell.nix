{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell rec {
  # buildInputs = [
  # ];
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    openssl
    pkg-config
    mold
    clang
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = 1;
}
