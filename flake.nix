{
  description = "Pie language";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
    tree-sitter-pie = {
      url = "path:./tree-sitter-pie";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        cargo2nix.follows = "cargo2nix";
        utils.follows = "utils";
      };
    };
  };

  outputs = { self, nixpkgs, utils, cargo2nix, tree-sitter-pie }:
   utils.lib.eachDefaultSystem (system:
     let
       pkgs = import nixpkgs {
         inherit system;
         overlays = [cargo2nix.overlays.default];
       };

       rustPkgs = pkgs.rustBuilder.makePackageSet {
         rustVersion = "1.71.0";
         packageFun = import ./Cargo.nix;
         # Generated Cargo.nix says
         # "unknown".tree-sitter-pie."0.0.1" = overridableMkRustCrate (profileName: rec {
         #   src = fetchCrateLocal workspaceSrc;
         # which is whole project directory
         packageOverrides = pkgs: pkgs.rustBuilder.overrides.all ++ [
           (pkgs.rustBuilder.rustLib.makeOverride {
             name = "tree-sitter-pie";
             overrideAttrs = drv: {
               src = ./tree-sitter-pie;
             };
           })
         ];
       };

     in rec {
       packages = {
         pie = (rustPkgs.workspace.pie {});
         default = packages.pie;
       };

       devShells = {
         default = rustPkgs.workspaceShell {
           packages = [
             pkgs.rust-analyzer
             pkgs.rustfmt
             pkgs.tree-sitter
             pkgs.gnuplot
             pkgs.nodejs_18
             pkgs.nodePackages.typescript-language-server
           ];
         };
         tree-sitter = tree-sitter-pie.devShells.${system}.default;
       };
     }
   );
}
