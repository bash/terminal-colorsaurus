#!/usr/bin/env bash

set -euo pipefail

rust_version=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages | map(select(.name == "terminal-colorsaurus")) | first | .rust_version')
rustup override set "$rust_version"
