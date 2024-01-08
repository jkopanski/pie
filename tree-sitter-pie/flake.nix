{
  description = "Pie tree-sitter grammar";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = { self, nixpkgs, utils, cargo2nix }:
   utils.lib.eachDefaultSystem (system:
     let
       pkgs = import nixpkgs {
         inherit system;
         overlays = [cargo2nix.overlays.default];
       };

       rustPkgs = pkgs.rustBuilder.makePackageSet {
         rustVersion = "1.71.0";
         packageFun = import ./Cargo.nix;
       };

     in rec {
       packages = {
         tree-sitter-pie = (rustPkgs.workspace.tree-sitter-pie {});
         default = packages.tree-sitter-pie;
       };

       devShells = {
         default = rustPkgs.workspaceShell {
           packages = [
             pkgs.rust-analyzer
             pkgs.tree-sitter
             pkgs.nodejs_18
             pkgs.nodePackages.typescript-language-server
           ];
           shellHook = ''
             export PS1="$PS1 \033[0;31m[tree-sitter]\033[0m "
           '';
         };
       };
     }
   );
}
