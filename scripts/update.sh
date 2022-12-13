#!/bin/ash
cd "$HOME/betterdns" || exit 1
git pull
cargo build --release
cp -f Corefile "$HOME"
cp -f dns_manifest "$HOME"
cp -f Rocket.toml "$HOME"
cp -f target/release/betterdns "$HOME"
export PATH="$PATH:$HOME/betterdns/target/release"
cd "$HOME" || exit 1
betterdns