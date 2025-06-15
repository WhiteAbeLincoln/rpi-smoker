# see https://github.com/oxalica/rust-overlay/issues/129#issuecomment-1575602642
{
  description = "Raspberry Pi Smoker";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [
        (import rust-overlay)
        # (self: super:
        #   let
        #     toolchain = super.rust-bin.stable.latest.default.override {
        #       extensions = [ "rust-src" ];
        #     };
        #   in
        #   {
        #     rustc = toolchain;
        #   })
      ];
      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      packages.${system} = rec {
        default = pkgs.callPackage ./server { };
        raspberrypi = pkgs.pkgsCross.armv7l-hf-multiplatform.callPackage ./server { };
        raspberrypi-static1 = pkgs.pkgsCross.armv7l-hf-multiplatform.pkgsStatic.callPackage ./server { };
        # we get linker issues when using the default pkgsCross.aarch64-multiplatform.pkgsStatic.callPackage
        # https://github.com/NixOS/nixpkgs/issues/264687#issuecomment-1826811183
        # instead manually define a static build for glibc (instead of musl)
        raspberrypi-static =
          let staticPkgs = import nixpkgs {
            inherit system overlays;
            crossSystem = {
              config = "aarch64-linux";
              rustc.config = "aarch64-unknown-linux-gnu";
            };
          };
          in
          staticPkgs.callPackage ./server { buildGNUStatic = true; };
      };

      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.nodejs
          pkgs.nodePackages.npm
          # wrapped version comes with nixpkgs' toolchain
          pkgs.rust-analyzer-unwrapped
          pkgs.rustc
          pkgs.rustup
        ];
        RUST_SRC_PATH = "${pkgs.rustc}/lib/rustlib/src/rust/library";
        shellHook = ''
          export PATH="$HOME/.cargo/bin:$PATH"
        '';
      };
    };
}
