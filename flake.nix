{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-21.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem
      (with flake-utils.lib.system; [ x86_64-linux ])
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          pythonDeps =
            (packages: with packages; [
            ]);
        in
        rec {
          packages = {
            pythonEnv = pkgs.python3.withPackages pythonDeps;
          };
          devShell = packages.pythonEnv.env;
        });
}
