# --- flake-parts/pkgs/default.nix
_: {
  perSystem = _: {
    packages = {
      # my-custom-package = pkgs.callPackage ./my-custom-package.nix { };
    };
  };
}
