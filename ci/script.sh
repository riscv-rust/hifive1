set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo check --target $TARGET --features 'board-hifive1'
    cargo check --target $TARGET --features 'board-lofive'
}

main
