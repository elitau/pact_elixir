all: libpact_matching libpact_mock_server pact_mock_server

libpact_matching:
	cd libpact_matching && cargo build && cargo test

pact_mock_server:
	cd pact_mock_server && cargo build && cargo test

libpact_mock_server:
	cd libpact_mock_server && cargo build && cargo test

libpact_matching_clean:
	cd libpact_matching && cargo clean

pact_mock_server_clean:
	cd pact_mock_server && cargo clean

libpact_mock_server_clean:
	cd libpact_mock_server && cargo clean

clean: libpact_matching_clean pact_mock_server_clean libpact_mock_server_clean
