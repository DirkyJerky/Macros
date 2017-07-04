#!/bin/bash
set -e

cd controller/
cargo build --release
cd ..
ln ./controller/target/release/controller ./control
