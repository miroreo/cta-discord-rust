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
        openssl
        pandoc
        pkg-config
        postgresql_16
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
            name = "PKG_CONFIG_PATH";
            value = "${pkgs.openssl.dev}/lib/pkgconfig";
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
