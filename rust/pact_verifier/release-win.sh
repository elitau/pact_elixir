#!/bin/bash
cargo clean
cargo build --release
gzip -c target/release/pact_verifier.dll > target/release/pact_verifier-windows-x86_64-$1.dll.gz
gzip -c target/release/pact_verifier.lib > target/release/pact_verifier-windows-x86_64-$1.lib.gz
