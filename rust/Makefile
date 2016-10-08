all: pact_matching pact_mock_server pact_mock_server_cli pact_verifier pact_verifier_cli pact_consumer

pact_matching:
	cd pact_matching && cargo build && cargo test

pact_mock_server_cli:
	cd pact_mock_server_cli && cargo build && cargo test

pact_mock_server:
	cd pact_mock_server && cargo build && cargo test

pact_verifier:
	cd pact_verifier && cargo build && cargo test

pact_verifier_cli:
	cd pact_verifier_cli && cargo build && cargo test

pact_consumer:
	cd pact_consumer && cargo build && cargo test

pact_matching_clean:
	cd pact_matching && cargo clean

pact_mock_server_cli_clean:
	cd pact_mock_server_cli && cargo clean

pact_mock_server_clean:
	cd pact_mock_server && cargo clean

pact_verifier_clean:
	cd pact_verifier && cargo clean

pact_verifier_cli_clean:
	cd pact_verifier_cli && cargo clean

pact_consumer_clean:
	cd pact_consumer && cargo clean

clean: pact_matching_clean pact_mock_server_cli_clean pact_mock_server_clean pact_verifier_clean pact_verifier_cli_clean pact_consumer_clean
