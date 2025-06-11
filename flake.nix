{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      ...
    }:
    let
      forSystems = f: nixpkgs.lib.attrsets.genAttrs nixpkgs.lib.systems.flakeExposed (system: f system);
      pkgs' = system: nixpkgs.legacyPackages.${system};
    in
    {
      devShells = forSystems (
        system:
        let
          pkgs = pkgs' system;
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              corepack_latest
              nodejs
              cargo
              clippy
            ];
          };
        }
      );
      packages = forSystems (
        system:
        let
          pkgs = pkgs' system;
        in
        {
          fmt = pkgs.treefmt.withConfig {
            runtimeInputs = with pkgs; [
              nixfmt-rfc-style
              nodePackages.prettier
              rustfmt
            ];
            settings = {
              tree-root-file = ".git/index";
              formatter = {
                nixfmt = {
                  command = "nixfmt";
                  includes = [ "*.nix" ];
                };
                rustfmt = {
                  command = "rustfmt";
                  options = [
                    "--edition"
                    "2018"
                  ];
                  includes = [ "*.rs" ];
                };
                prettier = {
                  command = "prettier";
                  options = [ "--write" ];
                  excludes = [ "pnpm-lock.yaml" ];
                  includes = [
                    "*.css"
                    "*.html"
                    "*.js"
                    "*.json"
                    "*.jsx"
                    "*.md"
                    "*.mdx"
                    "*.scss"
                    "*.ts"
                    "*.yaml"
                    "*.yml"
                    "*.vue"
                  ];
                };
              };
            };
          };
        }
      );
      formatter = forSystems (
        system:
        let
          pkgs = pkgs' system;
        in
        self.outputs.packages.${system}.fmt
      );
    };
}
