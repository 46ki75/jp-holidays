default:
    @just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Hermetic tests only: lib unit tests + doctests. No network.
test:
    cargo test --workspace

# Live tier: exercises the network `fetch` path (jp-holidays-lib `fetch` feature).
test-live:
    cargo test -p jp-holidays-lib --features fetch -- --ignored

# Instrumented hermetic test run (no report yet).
test-cov:
    cargo llvm-cov --no-report --workspace

# AI-friendly per-file table (drop 100% files) + uncovered line numbers.
coverage: test-cov
    cargo llvm-cov report --show-missing-lines --color=always 2>&1 | grep -v " 100.00%"

# Local HTML drilldown.
coverage-html: test-cov
    cargo llvm-cov report --html --open

# CI / Codecov upload.
coverage-ci: test-cov
    cargo llvm-cov report --lcov --output-path lcov.info

# Generate the static JSON API site into ./dist.
build-site out_dir="dist":
    cargo run -p jp-holidays -- --out-dir {{out_dir}}

ci: fmt-check lint test
