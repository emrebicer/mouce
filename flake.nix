{
  description = "Mouce development flake";

  inputs = {
    nixpkgs-unstable.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs, nixpkgs-unstable, ... }@inputs: 

    let 
      system = "x86_64-linux";
      pkgs = nixpkgs-unstable.legacyPackages.${system};
    in {
        devShells.${system}.default = pkgs.mkShellNoCC {
          buildInputs = with pkgs; [
              gcc13
              xorg.libX11
              xorg.libXtst
              cargo
              rustc
              rust-analyzer
              clippy
          ];
        };

    };
}
