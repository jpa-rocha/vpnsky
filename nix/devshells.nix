{
  age,
  git,
  mkShell,
  scripts,
  self,
  sops,
  system,
  cargo,
  rust-analyzer,
  ...
}:
{
  default = mkShell {
    packages = [
      age
      cargo
      git
      rust-analyzer
      scripts.lint
      sops
    ];

    inherit (self.checks.${system}.pre-commit-check) shellHook;
  };
}
