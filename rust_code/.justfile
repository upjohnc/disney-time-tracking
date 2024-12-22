default:
    just --list

run:
    cargo run

build:
    cargo build -r
    mv target/release/disney_tracking .

