{
    description = "egui project devshell";

    inputs = {
      nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
      rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, nixpkgs, rust-overlay }:
      let
        system = "x86_64-linux";
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        eguiLibs = with pkgs; [
          libxkbcommon
          wayland
          libGL
          vulkan-loader
          fontconfig
        ];
      in {
        devShells.${system}.default = pkgs.mkShell {
          packages = with pkgs; [
            (rust-bin.stable.latest.default.override {
              targets = [ "wasm32-unknown-unknown" ];
            })
            pkg-config
            trunk
          ] ++ eguiLibs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath eguiLibs;
        };
      };
  }
