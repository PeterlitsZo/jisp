_default:
  @just --list

# Run clippy.
clippy:
  cargo clippy -- -D warnings

# Run coverage.
coverage arg='help':
  bash scripts/coverage.sh {{arg}}

# List todos in source code.
list-todos:
  bash scripts/list-todos.sh