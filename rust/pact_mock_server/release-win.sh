#!/bin/bash
cargo clean
cargo build --release
gzip -c ../target/release/pact_mock_server.dll > ../target/release/pact_mock_server-windows-x86_64-$1.dll.gz
gzip -c ../target/release/pact_mock_server.lib > ../target/release/pact_mock_server-windows-x86_64-$1.lib.gz

