{
  description = "A basic flake with a shell";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit overlays; system = "x86_64-linux"; };
      rustVersion = pkgs.rust-bin.stable.latest.default;
    in
    {
      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;
      devShells.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.mkShell {
        shellHook = ''
          cargo install cargo-binutils
          cargo install elf2uf2-rs
        '';
        packages = with nixpkgs.legacyPackages.x86_64-linux; [
          pkg-config
          systemd
          (pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "llvm-tools-preview" ];
            targets = [ "thumbv6m-none-eabi" ];
          })
        ];
      };
    };
}
