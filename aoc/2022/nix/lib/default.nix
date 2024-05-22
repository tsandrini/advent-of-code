# --- nix/lib/default.nix
{
  pkgs,
  lib,
  inputs,
  projectPath ? ./..,
  ...
}:
let
  inherit (bootstrap) mapModules';

  bootstrap = import ./_bootstrap-lib.nix { inherit lib; };

  aoc2022 = lib.makeExtensible (
    self:
    with self;
    mapModules' ./. (
      file:
      import file {
        inherit
          pkgs
          lib
          self
          inputs
          projectPath
          ;
      }
    )
  );
in
aoc2022.extend (_self: super: lib.foldr (a: b: a // b) { } (lib.attrValues super))
