default:
    just --list

test-package name *args:
    #!/usr/bin/env bash
    set -euxo pipefail
    CARGO_TARGET_DIR=$(mktemp -d); export CARGO_TARGET_DIR
    trap 'rm -rf "$CARGO_TARGET_DIR"' EXIT
    cargo package -p "{{name}}" {{args}}
    (cd $CARGO_TARGET_DIR/package/{{name}}-*/ && cargo test)

# Updates the lockfile using the MSRV-aware resolver
update-cargo-lock:
    CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback cargo +nightly -Zmsrv-policy generate-lockfile
