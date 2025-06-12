#!/bin/sh

new() {
    if [ -z "$1" ]; then
        echo "Usage: new <library-name>"
        return 1
    fi

    cargo new --lib "$1" || return 1
    cd "$1" || return 1
    touch src/main.rs || return 1
    : >src/lib.rs
}
