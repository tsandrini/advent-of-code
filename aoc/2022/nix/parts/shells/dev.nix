# --- nix/parts/shells/dev.nix
{
  pkgs,
  treefmt,
  devenv-root,
  ...
}:
{
  # DEVENV:  Fast, Declarative, Reproducible, and Composable Developer
  # Environments using Nix developed by Cachix. For more information refer to
  #
  # - https://devenv.sh/
  # - https://github.com/cachix/devenv

  # --------------------------
  # --- ENV & SHELL & PKGS ---
  # --------------------------
  packages = with pkgs; [
    # -- NIX UTILS --
    nix-output-monitor # Processes output of Nix commands to show helpful and pretty information
    nixfmt-rfc-style # An opinionated formatter for Nix
    # NOTE Choose a different formatter if you'd like to
    # nixfmt # An opinionated formatter for Nix
    # alejandra # The Uncompromising Nix Code Formatter
    nh # Yet another nix cli helper

    # -- GIT RELATED UTILS --
    commitizen # Tool to create committing rules for projects, auto bump versions, and generate changelogs
    cz-cli # The commitizen command line utility
    # fh # The official FlakeHub CLI
    gh # GitHub CLI tool

    # -- BASE LANG UTILS --
    markdownlint-cli # Command line interface for MarkdownLint
    typos # Source code spell checker
    treefmt # one CLI to format the code tree

    # -- (YOUR) EXTRA PKGS --
    openssl
  ];

  enterShell = ''
    # Welcome splash text
    echo ""; echo -e "\e[1;37;42mWelcome to the aoc2022 devshell!\e[0m"; echo ""
  '';

  # ---------------
  # --- SCRIPTS ---
  # ---------------
  scripts = {
    "rename-project".exec = ''
      find $1 \( -type d -name .git -prune \) -o -type f -print0 | xargs -0 sed -i "s/aoc2022/$2/g"
    '';
  };

  # -----------------
  # --- LANGUAGES ---
  # -----------------
  languages.nix.enable = true;
  languages.rust = {
    enable = true;
    channel = "nixpkgs";

    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  # ----------------------------
  # --- PROCESSES & SERVICES ---
  # ----------------------------

  # ------------------
  # --- CONTAINERS ---
  # ------------------
  devcontainer.enable = true;

  # ----------------------
  # --- BINARY CACHING ---
  # ----------------------

  # ------------------------
  # --- PRE-COMMIT HOOKS ---
  # ------------------------
  # NOTE All available hooks options are listed at
  # https://devenv.sh/reference/options/#pre-commithooks
  # NOTE disabled due to not being in the root of the project
  pre-commit = {
    hooks = {
      treefmt.enable = true;
      treefmt.package = treefmt;

      nil.enable = true; # Nix Language server, an incremental analysis assistant for writing in Nix.
      markdownlint.enable = true; # Markdown lint tool
      typos.enable = true; # Source code spell checker
      # editorconfig-checker.enable = true; # A tool to verify that your files are in harmony with your .editorconfig

      actionlint.enable = true; # GitHub workflows linting
      commitizen.enable = true; # Commitizen is release management tool designed for teams.

      #   clippy.enable = true;
    };
  };

  # --------------
  # --- FLAKES ---
  # --------------
  devenv.flakesIntegration = true;

  # This is currently needed for devenv to properly run in pure hermetic
  # mode while still being able to run processes & services and modify
  # (some parts) of the active shell.
  devenv.root =
    let
      devenvRootFileContent = builtins.readFile devenv-root.outPath;
    in
    pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

  # ---------------------
  # --- MISCELLANEOUS ---
  # ---------------------
  difftastic.enable = true;
}
