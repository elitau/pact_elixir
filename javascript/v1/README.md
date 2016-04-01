To run the javascript examples, the mock server DLL needs to be built using `cargo build`
in the `rust/v1/libpact_v1_mock_server` directory.

1. run `npm install`
2. run `npm run simple_pact`

To change the log level, use the `RUST_LOG` environment variable. I.e., to set
debug level: `RUST_LOG=debug npm run simple_pact`
