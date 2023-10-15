#!/bin/bash

# Bash 'Strict Mode'
# http://redsymbol.net/articles/unofficial-bash-strict-mode
# https://github.com/xwmx/bash-boilerplate#bash-strict-mode
set -o nounset
set -o errexit
set -o pipefail
IFS=$'\n\t'

rustup default stable

git submodule update

cd site
curl -L https://github.com/getzola/zola/releases/download/v0.17.2/zola-v0.17.2-x86_64-unknown-linux-gnu.tar.gz | tar xvz
./zola build --output-dir ../build
cd ..

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
mv book/html build/book
