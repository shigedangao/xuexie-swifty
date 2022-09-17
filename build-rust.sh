#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"
# Build the project for the desired platforms:
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-darwin
mkdir -p ./target/universal-macos/debug

lipo \
    ./target/aarch64-apple-darwin/debug/libxuexi_swifty.a \
    ./target/x86_64-apple-darwin/debug/libxuexi_swifty.a -create -output \
    ./target/universal-macos/debug/libxuexi_swifty.a

cargo build --target aarch64-apple-ios

cargo build --target x86_64-apple-ios
cargo build --target aarch64-apple-ios-sim
mkdir -p ./target/universal-ios/debug

lipo \
    ./target/aarch64-apple-ios-sim/debug/libxuexi_swifty.a \
    ./target/x86_64-apple-ios/debug/libxuexi_swifty.a -create -output \
    ./target/universal-ios/debug/libxuexi_swifty.a
