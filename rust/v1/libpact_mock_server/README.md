# Pact Mock Server library

This library implements the in-process mock server for matching HTTP requests and generating responses off a pact file.

[Online rust docs](http://www.pact.io/reference/rust/libpact_mock_server-docs-latest/pact_mock_server/)

For an example of calling these functions, have a [look at the JavaScript reference](blob/master/javascript/v1/README.md).

There are a number of exported functions using C bindings for controlling the mock server. These can be used in any
language that supports C bindings.

## [create_mock_server](http://www.pact.io/reference/rust/libpact_mock_server-docs-latest/pact_mock_server/fn.create_mock_server.html)

This function will create a new mock server given a Pact in JSON. The port that the mock server is allocated is returned
and can be provided to the test code as the address of the provider.

## [mock_server_matched](http://www.pact.io/reference/rust/libpact_mock_server-docs-latest/pact_mock_server/fn.mock_server_matched.html)

Simple function that returns a boolean value given the port number of the mock service. This value will be true if all
the expectations of the pact that the mock server was created with have been met. It will return false if any request did
not match, an un-recognised request was received or an expected request was not received.

## [mock_server_mismatches](http://www.pact.io/reference/rust/libpact_mock_server-docs-latest/pact_mock_server/fn.mock_server_mismatches.html)

This returns all the mismatches, un-expected requests and missing requests in JSON format, given the port number of the
mock server.

** IMPORTANT NOTE:** The JSON string for the result is allocated on the rust heap, and will have to be freed once the
code using the mock server is complete. The `cleanup_mock_server` function is provided for this purpose. If the mock
server is not cleaned up properly, this will result in memory leaks as the rust heap will not be reclaimed.

## [cleanup_mock_server](http://www.pact.io/reference/rust/libpact_mock_server-docs-latest/pact_mock_server/fn.cleanup_mock_server.html)

This function will try terminate the mock server with the given port number and cleanup any memory allocated for it by
the `mock_server_mismatches` function. Returns `true`, unless a mock server with the given port number does not exist,
or the function fails in some way.

**NOTE:** Although `close()` on the listerner for the mock server is called, this does not currently work and the
listerner will continue handling requests. In this case, it will always return a 404 once the mock server has been
cleaned up.
