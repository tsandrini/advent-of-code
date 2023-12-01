# --- nix/parts/shells/dev.nix
{
  pkgs,
  treefmt,
  ...
}: {
  packages = with pkgs; [
    # -- greeting --
    cowsay
    fortune
    lolcat
    # -- nix --
    nil # LSP
    alejandra # formatting
    statix # static code analysis
    deadnix # find dead nix code
    nix-output-monitor # readable derivation outputs
    # -- misc --
    markdownlint-cli # markdown linting
    nodePackages.prettier
    treefmt

    commitizen
    cz-cli

    # ocaml
    ocamlPackages.merlin
    ocamlPackages.merlin-lib
    ocamlPackages.utop
    ocamlPackages.ocp-indent
    ocamlPackages.ocamlformat_0_26_0
    opam
    dune_3
  ];

  languages.nix.enable = true;
  languages.python.enable = true;
  languages.ocaml.enable = true;
  difftastic.enable = true;
  devcontainer.enable = true; # if anyone needs it
  devenv.flakesIntegration = true;

  pre-commit = {
    hooks = {
      treefmt.enable = true;
      # Everything below is stuff that is missing from treefmt
      nil.enable = true;
      markdownlint.enable = true;
      actionlint.enable = true;
      commitizen.enable = true;
    };
    settings = {
      treefmt.package = treefmt;
    };
  };

  scripts = {
    "rename-project".exec = ''
      find $1 \( -type d -name .git -prune \) -o -type f -print0 | xargs -0 sed -i "s/aoc2023/$2/g"
    '';
  };

  enterShell = ''
    echo ""
    echo "~~ Welcome to the aoc2023 devshell! ~~

    [Fortune of the Day] $(fortune)" | cowsay -W 120 -T "U " | lolcat -F 0.3 -p 10 -t
    echo ""
  '';
}
