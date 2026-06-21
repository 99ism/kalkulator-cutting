{
    description = "egui project devshell";

    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

    outputs = { self, nixpkgs }:
      let
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
        eguiLibs = with pkgs; [
          libxkbcommon
          wayland
          libGL
          vulkan-loader
          fontconfig
        ];
      in {
        devShells.${system}.default = pkgs.mkShell {
          packages = with pkgs; [ pkg-config ] ++ eguiLibs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath eguiLibs;
        };
      };
  }
