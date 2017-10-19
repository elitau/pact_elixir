#!/bin/bash -xe
cargo clean
cargo build --release
cargo build --release --target x86_64-apple-ios
gzip -c ../target/release/pact_mock_server_cli > ../target/release/pact_mock_server_cli-osx-x86_64-$1.gz
cargo build --release --target x86_64-apple-ios
gzip -c ../target/x86_64-apple-ios/release/pact_mock_server_cli > ../target/x86_64-apple-ios/release/pact_mock_server_cli-ios-x86_64-$1.gz
