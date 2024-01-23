#!/usr/bin/env bash

set -e

metadata=$(cargo metadata --format-version 1 --no-deps | jq '.packages | map(select(.name == "terminal-colorsaurus")) | first | .metadata.docs.rs')
features=$(echo "$metadata" | jq -r '.features | join(",")')

export RUSTDOCARGS=$(echo "$metadata" | jq -r '.["rustdoc-args"] | join(" ")')
cargo +nightly doc -Zunstable-options -Zrustdoc-scrape-examples --features "$features"
