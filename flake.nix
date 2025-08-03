{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [
            (import rust-overlay)
          ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          dependencies = {
            LIBCLANG_PATH = "${pkgs.llvmPackages_16.libclang.lib}/lib";
            LD_LIBRARY_PATH = "${pkgs.nix}/lib";
            nativeBuildInputs = with pkgs; [
              pkg-config
            ];
            buildInputs = with pkgs; [
              nix
              boost
              brotli
              libarchive
              libblake3
              libcpuid
              libsodium
              nlohmann_json
              openssl
              libseccomp
              sqlite
              curl
              libgit2
              pcre2
              libunistring
              aws-crt-cpp
              llhttp
              bzip2
              lowdown
              editline
              rust-bin.stable.latest.default
            ];
          };
          customBuildRustCrateForPkgs = pkgs: pkgs.buildRustCrate.override {
            stdenv = pkgs.clang16Stdenv;
            defaultCrateOverrides = pkgs.defaultCrateOverrides // rec {
              base = attrs: dependencies;
              nix-exprc-sys = base;
              nix-flakec-sys = base;
              nix-storec-sys = base;
              nix-utilc-sys = base;
              nix-mainc-sys = base;
            };
          };
          generatedBuild = pkgs.callPackage ./Cargo.nix {
            buildRustCrateForPkgs = customBuildRustCrateForPkgs;
          };
        in
        with pkgs;
        {
          devShells.default = pkgs.mkShell.override
            {
              stdenv = pkgs.clang16Stdenv;
            }
            dependencies;

          packages.prebuilt = pkgs.symlinkJoin {
            name = "all-workspace-members";
            paths =
              let members = builtins.attrValues generatedBuild.workspaceMembers;
              in builtins.map (m: m.build.override { runTests = true; }) members;
          };
        });
}
