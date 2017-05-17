#!/bin/bash -x
cargo clean
cargo build --release
gzip -c target/release/libpact_verifier.so > target/release/libpact_verifier-linux-x86_64-$1.so.gz
