all: libpact_v1_matching libpact_v1_mock_server pact_v1_mock_server

libpact_v1_matching:
	cd libpact_v1_matching && cargo build && cargo test

pact_v1_mock_server:
	cd pact_v1_mock_server && cargo build && cargo test

libpact_v1_mock_server:
	cd libpact_v1_mock_server && cargo build && cargo test

libpact_v1_matching_clean:
	cd libpact_v1_matching && cargo clean

pact_v1_mock_server_clean:
	cd pact_v1_mock_server && cargo clean

libpact_v1_mock_server_clean:
	cd libpact_v1_mock_server && cargo clean

clean: libpact_v1_matching_clean pact_v1_mock_server_clean libpact_v1_mock_server_clean
