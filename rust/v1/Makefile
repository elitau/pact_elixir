all: libpact_v1_models libpact_v1_matchers libpact_v1_mock_server pact_v1_mock_server

libpact_v1_models:
	cd libpact_v1_models && cargo build && cargo test

libpact_v1_matchers:
	cd libpact_v1_matchers && cargo build && cargo test

pact_v1_mock_server:
	cd pact_v1_mock_server && cargo build && cargo test

libpact_v1_mock_server:
	cd libpact_v1_mock_server && cargo build && cargo test
