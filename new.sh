#!/bin/sh

new() {
    if [ -z "$1" ]; then
        echo "Usage: new <library-name>"
        return 1
    fi

    CURRENT_DIR_NAME=$(basename "$PWD")

    if [ "$CURRENT_DIR_NAME" = "piscine-rust" ]; then
        TARGET_DIR="./"
    else
        TARGET_DIR="../"
    fi

    if [ ! -d "$TARGET_DIR" ]; then
        echo "Error: piscine-rust directory not found at $TARGET_DIR"
        return 1
    fi

    cd "$TARGET_DIR" || return 1
    cargo new --lib "$1" || return 1
    cd "$1" || return 1
    touch src/main.rs || return 1
    : > src/lib.rs
}
