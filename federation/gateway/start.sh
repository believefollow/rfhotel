#!/bin/bash

function cleanup {
    kill "$ACCOUNTS_PID"
    kill "$PRODUCTS_PID"
    kill "$REVIEWS_PID"
}
trap cleanup EXIT

cargo build --bin axum-customers
cargo build --bin axum-managers
cargo build --bin axum-rooms

cargo run --bin axum-customers  &
ACCOUNTS_PID=$!

cargo run --bin axum-managers &
PRODUCTS_PID=$!

cargo run --bin axum-rooms &
REVIEWS_PID=$!

sleep 3

node index.js
