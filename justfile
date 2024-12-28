_default:
  @just --list

# Run clippy.
clippy:
  cargo clippy -- -D warnings

# Run coverage.
coverage arg='help':
  bash scripts/coverage.sh {{arg}}
