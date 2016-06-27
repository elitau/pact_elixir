all: libpact_matching libpact_mock_server pact_mock_server_cli

libpact_matching:
	cd libpact_matching && cargo build && cargo test

pact_mock_server_cli:
	cd pact_mock_server_cli && cargo build && cargo test

libpact_mock_server:
	cd libpact_mock_server && cargo build && cargo test

libpact_matching_clean:
	cd libpact_matching && cargo clean

pact_mock_server_cli_clean:
	cd pact_mock_server_cli && cargo clean

libpact_mock_server_clean:
	cd libpact_mock_server && cargo clean

clean: libpact_matching_clean pact_mock_server_cli_clean libpact_mock_server_clean
