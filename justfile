
[private]
default:
    @just --list --unsorted

dev:
    cargo run

run +args="show":
    cargo run --release -- {{args}}

publish:
    ./scripts/publish.sh
