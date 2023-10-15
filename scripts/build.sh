#!/bin/bash

# Bash 'Strict Mode'
# http://redsymbol.net/articles/unofficial-bash-strict-mode
# https://github.com/xwmx/bash-boilerplate#bash-strict-mode
set -o nounset
set -o errexit
set -o pipefail
IFS=$'\n\t'

rustup default stable

# for faster installation
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall -y \
    mdbook \
    mdbook-admonish \
    mdbook-catppuccin \
    mdbook-linkcheck \
    mdbook-mermaid \
    mdbook-emojicodes \
    mdbook-svgbob2

mdbook build
