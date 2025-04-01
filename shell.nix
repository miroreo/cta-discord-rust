{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
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
    # rustChannels.stable.rust-src

    # (
    #   pkgs.rust-bin.stable.latest.rust.override {
    #     extensions = [ "rust-src" ];
    #   }
    # )
    
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = 1;
}
