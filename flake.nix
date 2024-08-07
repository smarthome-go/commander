{
  description = "Environment for developing and deploying the Commander project.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        # rustToolchain = pkgs.pkgsBuildHost.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
        #     # extensions = [ "rust-src" ];
        #     targets = [ "x86_64-unknown-linux-musl" ];
        # });
      in {
        devShells.default = pkgs.mkShell {
          name = "Commander Dev";

          buildInputs = with pkgs; [
            # Rust toolchain
            # (rustToolchain.override {
            #   extensions = ["rust-src" "rust-std" "rust-analyzer"];
            # })

            cargo-cross

            # Misc
            ripgrep
            #openssl
            #pkg-config
            #perl
            #findutils
          ];

          shellHook = ''
            # if running from zsh, reenter zsh
            if [[ $(ps -e | grep $PPID) == *"zsh" ]]; then
              zsh
              exit
            fi
          '';
        };

        formatter = nixpkgs.legacyPackages.${system}.alejandra;
      }
    );
}
