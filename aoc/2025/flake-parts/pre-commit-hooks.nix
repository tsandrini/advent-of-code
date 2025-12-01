# --- flake-parts/pre-commit-hooks.nix
{ inputs, ... }:
{
  imports = with inputs; [ pre-commit-hooks.flakeModule ];

  perSystem = _: {
    pre-commit.settings = {
      excludes = [
        "flake.lock" # NOTE: prettier thinks this is json >.< prettier baka!!!
        "src/template.txt"
      ];

      hooks = {
        # --- Nix ---
        deadnix.enable = true; # Find and remove unused code in .nix source files
        nil.enable = true; # Nix Language server, an incremental analysis assistant for writing in Nix.
        nixfmt-rfc-style.enable = true; # An opinionated formatter for Nix
        statix.enable = true; # Lints and suggestions for the nix programming language

        # --- Shell ---
        shellcheck.enable = true; # Shell script analysis tool
        shfmt.enable = true; # Shell parser and formatter

        # --- Misc ---
        markdownlint.enable = true; # Markdown lint tool
        editorconfig-checker.enable = true; # .editorconfig file checker
        editorconfig-checker.excludes = [
          "\\.nix$"
          "\\.md$"
          "\\.envrc$"
        ];
        typos.enable = true; # Source code spell checker
        # prettier.enable = true; # Prettier is an opinionated code formatter
        # jsonfmt.enable = true; # Formatter for JSON files

        # --- fs utils ---
        check-added-large-files.enable = true;
        check-executables-have-shebangs.enable = true;
        end-of-file-fixer.enable = true;
        mixed-line-endings.enable = true;
        trim-trailing-whitespace.enable = true;

        # --- VCS ---
        # actionlint.enable = true; # GitHub workflows linting
        commitizen.enable = true; # Commitizen is release management tool designed for teams.
        ripsecrets.enable = true; # A tool to prevent committing secret keys into your source code
      };
    };
  };
}
