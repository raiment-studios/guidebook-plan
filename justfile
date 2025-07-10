
[private]
default:
    @just --list --unsorted

run:
    cargo build
    ./target/debug/guidebook-plan

