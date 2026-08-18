@_default:
  just --list

# Build the full package, including any checks and documentation
run-all: update-quarto-theme format-all check-all test-all build-all

# Run all formatters
format-all: format-rust format-md

# Run all checks
check-all: check-spelling check-urls check-fmt check-cargo check-clippy

# Run all tests
test-all: test-rust

# Run all build recipes
build-all: build-rust-docs build-contributors build-readme build-website build-package

# List all TODO items in the repository
list-todos:
  grep -R -n \
    --exclude="*.code-snippets" \
    --exclude-dir=.quarto \
    --exclude-dir=.git \
    --exclude=justfile \
    --exclude=*.pyc \
    --exclude=_site \
    "TODO" .

# Install the pre-commit hooks
install-precommit:
    uvx pre-commit install
    uvx pre-commit autoupdate
    uvx pre-commit run --all-files

# Update the Quarto seedcase-theme extension
update-quarto-theme:
  # Add theme if it doesn't exist, update if it does
  quarto update seedcase-project/seedcase-theme --no-prompt

# Check for spelling errors in files
check-spelling:
  uvx typos --config .config/typos.toml

# Check that URLs work
check-urls:
  # Ignore pre-commit.ci and GitHub URLs, since they are often block "bot" requests
  lychee . \
    --verbose \
    --exclude 'pre-commit\.ci' \
    --exclude 'github\.com' \
    --exclude-path "_badges.qmd"

# Checks and lints with clippy
check-clippy:
  # Stricter linting
  cargo clippy -- -W clippy::pedantic

# Checks package and dependencies
check-cargo:
  cargo check

# Checks formatting with rustfmt
check-fmt:
  cargo +nightly fmt --check -- --config-path .config/rustfmt.toml

# Format the code and fix issues
format-rust:
  cargo fix --allow-dirty
  cargo clippy --fix --allow-dirty -- -W clippy::pedantic
  cargo +nightly fmt -- --config-path .config/rustfmt.toml

# Format Markdown files
format-md:
  # Use both rumdl and panache, for different purposes
  uvx rumdl fmt --silent
  uvx --from panache-cli panache format . --quiet

# Run the tests in the `src/` or `tests/` directories
test-rust:
  cargo test

# Build the code documentation
build-rust-docs:
  cargo doc

# Re-build the README file from the Quarto version
build-readme:
  uvx --from quarto quarto render README.qmd --to gfm

# Generate a Quarto include file with the contributors
build-contributors:
  sh ./tools/get-contributors.sh seedcase-project/zen-do > docs/includes/_contributors.qmd

# Build the website using Quarto
build-website:
  uvx --from quarto quarto render

# Build the package
build-package:
  cargo build

# Preview the website with automatic reload on changes
preview-website:
  quarto preview

# Check for and apply updates from the template
update-from-template:
  uvx copier update --defaults

# Reset repo changes to match the template
reset-from-template:
  uvx copier recopy --defaults
