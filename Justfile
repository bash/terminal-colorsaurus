default:
    just --list

test-package name *args:
    #!/usr/bin/env bash
    set -euxo pipefail
    CARGO_TARGET_DIR=$(mktemp -d); export CARGO_TARGET_DIR
    trap 'rm -rf "$CARGO_TARGET_DIR"' EXIT
    cargo package -p "{{name}}" {{args}}
    (cd $CARGO_TARGET_DIR/package/{{name}}-*/ && cargo test)

check: clippy check-no-default-features check-unsupported

clippy:
    cargo clippy --workspace --tests --all-features --all-targets

check-no-default-features:
    cargo clippy -p terminal-colorsaurus --no-default-features

check-unsupported:
    RUSTFLAGS='--cfg terminal_colorsaurus_test_unsupported -Dwarnings' cargo clippy --workspace

doc:
    cargo +nightly docs-rs -p terminal-colorsaurus

update-locked-deps:
    CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback cargo +nightly -Zmsrv-policy generate-lockfile

update-python-ci:
    (cd crates/pycolorsaurus && maturin generate-ci github > ../../.github/workflows/python.yml)
    git apply .github/workflows/python.yml.patch
