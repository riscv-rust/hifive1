#!/usr/bin/env bash

set -euxo pipefail

cargo check --target $TARGET --features 'board-hifive1'
cargo check --target $TARGET --features 'board-hifive1-revb'
cargo check --target $TARGET --features 'board-redv'
cargo check --target $TARGET --features 'board-lofive'
cargo check --target $TARGET --features 'board-lofive-r1'
