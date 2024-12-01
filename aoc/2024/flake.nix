# --- flake.nix
{
  description = "TODO Add description of your new project";

  inputs = {
    # --- BASE DEPENDENCIES ---
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    # --- YOUR DEPENDENCIES --- 
    systems.url = "github:nix-systems/default";
  };

  # NOTE Here you can add additional binary cache substituers that you trust.
  # There are also some sensible default caches commented out that you
  # might consider using, however, you are advised to doublecheck the keys.
  nixConfig = {
    extra-trusted-public-keys = [
      # "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      # "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
    extra-substituters = [
      # "https://cache.nixos.org"
      # "https://nix-community.cachix.org/"
    ];
  };

  outputs =
    inputs@{ flake-parts, ... }:
    let
      inherit (inputs.nixpkgs) lib;
      inherit (import ./flake-parts/_bootstrap.nix { inherit lib; }) loadParts;
    in
    flake-parts.lib.mkFlake { inherit inputs; } {

      # We recursively traverse all of the flakeModules in ./flake-parts and
      # import only the final modules, meaning that you can have an arbitrary
      # nested structure that suffices your needs. For example
      #
      # - ./flake-parts
      #   - modules/
      #     - nixos/
      #       - myNixosModule1.nix
      #       - myNixosModule2.nix
      #       - default.nix
      #     - home-manager/
      #       - myHomeModule1.nix
      #       - myHomeModule2.nix
      #       - default.nix
      #     - sharedModules.nix
      #   - pkgs/
      #     - myPackage1.nix
      #     - myPackage2.nix
      #     - default.nix
      #   - mySimpleModule.nix
      #   - _not_a_module.nix
      imports = loadParts ./flake-parts;
    };
}
