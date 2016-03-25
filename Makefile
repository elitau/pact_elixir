all: libpact libpact_matchers pact_mock_server

libpact:
	cd rust/v1/libpact && cargo build && cargo test

libpact_matchers:
	cd rust/v1/libpact_matchers && cargo build && cargo test

pact_mock_server:
	cd rust/v1/pact_mock_server && cargo build && cargo test
