{
  inputs,
  lib,
  ...
}: {
  imports = [
    inputs.devshell.flakeModule
  ];

  config.perSystem = {pkgs, ...}:
    with pkgs; let
      deps = [
        postgresql_16
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

    in {

      config.devshells.default = {
        imports = [
          "${inputs.devshell}/extra/language/c.nix"
          # "${inputs.devshell}/extra/language/rust.nix"
        ];

        devshell.packages = deps;
        env = [
          {
            name = "LD_LIBRARY_PATH";
            value = lib.makeLibraryPath deps;
          }
          {
            name = "CACHE_DIRECTORY";
            value = "./.cache";
          }
          {
            name = "PKG_CONFIG_PATH";
            value = "${pkgs.openssl.dev}/lib/pkgconfig";
          }
          {
            name = "RUST_SRC_PATH";
            value = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          }
          {
            name = "RUST_BACKTRACE";
            value = 1;
          }
        ];

        commands = with pkgs; [
          {
            package = sqlx-cli;
          }
          {
            name = "database:init";
            command = ''
              initdb -D .db;
            '';
          }
          {
            name = "database:sh";
            command = "psql -h `pwd` -d cta-discord";
          }
        ];

        serviceGroups.database.services.postgres = {
          command = ''
            postgres -D .db -k "$PWD" -c listen_addresses="" > db.log
          '';
        };

        # language.c = {
        #   libraries = lib.optional pkgs.stdenv.isDarwin pkgs.libiconv;
        # };
      };
    };
}
