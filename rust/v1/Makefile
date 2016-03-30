all: libpact_v1_matching libpact_v1_mock_server pact_v1_mock_server

libpact_v1_matching:
	cd libpact_v1_matching && cargo build && cargo test

pact_v1_mock_server:
	cd pact_v1_mock_server && cargo build && cargo test

libpact_v1_mock_server:
	cd libpact_v1_mock_server && cargo build && cargo test
