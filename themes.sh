#!/usr/bin/env bash

printf '\e]10;#5a4aae\a'
printf '\e]11;#170046\a'

cargo run --example theme

read -r

printf '\e]10;#170046\a'
printf '\e]11;#5a4aae\a'

cargo run --example theme

read -r

printf '\e]10;#6d5fc5\a'
printf '\e]11;#988df8\a'

cargo run --example theme

read -r

printf '\e]10;#988df8\a'
printf '\e]11;#6d5fc5\a'

cargo run --example theme
