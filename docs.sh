#!/usr/bin/env bash

set -euo pipefail

metadata=$(cargo metadata --format-version 1 --no-deps | jq '.packages | map(select(.name == "terminal-colorsaurus")) | first | .metadata.docs.rs')
features=$(echo "$metadata" | jq -r '.features | join(",")')

export RUSTDOCFLAGS
RUSTDOCFLAGS="$RUSTDOCFLAGS $(echo "$metadata" | jq -r '.["rustdoc-args"] | join(" ")')"
echo "+ RUSTDOCFLAGS=$RUSTDOCFLAGS" > /dev/stderr
echo "+ cargo doc ... --features $features" > /dev/stderr
cargo +nightly doc -Zunstable-options -Zrustdoc-scrape-examples --features "$features"
