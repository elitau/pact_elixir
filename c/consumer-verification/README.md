## Example use of the rust verification and mock server libraries from C.

Before you can run the C examples, the mock server DLL needs to be built using `cargo build`
in the `rust/libpact_mock_server` directory.

### Install libcurl

The C example uses libcurl, so the development library needs to be installed. On Ubuntu, you can use apt to install it.

    $ apt-get install libcurl4-openssl-dev

### configure/make dance

Next, for Linux and OSX, do the standard configure and make series of steps.

    $ ./configure
    checking for a BSD-compatible install... /usr/bin/install -c
    checking whether build environment is sane... yes
    checking for a thread-safe mkdir -p... /bin/mkdir -p
    checking for gawk... no
    checking for mawk... mawk
    checking whether make sets $(MAKE)... yes
    checking whether make supports nested variables... yes
    checking for gcc... gcc
    checking whether the C compiler works... yes
    checking for C compiler default output file name... a.out
    checking for suffix of executables...
    checking whether we are cross compiling... no
    checking for suffix of object files... o
    checking whether we are using the GNU C compiler... yes
    checking whether gcc accepts -g... yes
    checking for gcc option to accept ISO C89... none needed
    checking whether gcc understands -c and -o together... yes
    checking for style of include used by make... GNU
    checking dependency style of gcc... gcc3
    checking for gawk... (cached) mawk
    checking for curl-config... /usr/bin/curl-config
    checking for the version of libcurl... 7.47.0
    checking whether libcurl is usable... yes
    checking for curl_free... yes
    checking that generated files are newer than configure... done
    configure: creating ./config.status
    config.status: creating Makefile
    config.status: creating src/Makefile
    config.status: creating config.h
    config.status: config.h is unchanged
    config.status: executing depfiles commands
    ronald@ronald-VirtualBox:~/Development/pact-reference/c/consumer-verification$ make
    make  all-recursive
    make[1]: Entering directory '/home/ronald/Development/pact-reference/c/consumer-verification'
    Making all in src
    make[2]: Entering directory '/home/ronald/Development/pact-reference/c/consumer-verification/src'
    gcc -DHAVE_CONFIG_H -I. -I..     -g -O2 -MT main.o -MD -MP -MF .deps/main.Tpo -c -o main.o main.c
    mv -f .deps/main.Tpo .deps/main.Po
    gcc  -g -O2   -o consumer-verification main.o -L/usr/lib/x86_64-linux-gnu -lcurl -ldl
    make[2]: Leaving directory '/home/ronald/Development/pact-reference/c/consumer-verification/src'
    make[2]: Entering directory '/home/ronald/Development/pact-reference/c/consumer-verification'
    make[2]: Leaving directory '/home/ronald/Development/pact-reference/c/consumer-verification'
    make[1]: Leaving directory '/home/ronald/Development/pact-reference/c/consumer-verification'

Now you have an executable `src/consumer-verification` that links to the libpact_mock_server library.

## Running the tests

There are two tests. The basic test expects all requests to the verified, and the error test where there should be
validation errors. The src/consumer-verification executable takes 2 parameters: the test to run (basic or error) and the
path to the libpact_mock_server library.

    $ src/consumer-verification basic ../../rust/libpact_mock_server/target/debug/libpact_mock_server.so
    This is consumer-verification 0.0.0.
    Running basic pact test
    Mock server started on port 39263
    Executing request against http://localhost:39263/mallory?name=ron&status=good
    *   Trying 127.0.0.1...
    * Connected to localhost (127.0.0.1) port 39263 (#0)
    > GET /mallory?name=ron&status=good HTTP/1.1
    Host: localhost:39263
    Accept: */*

    < HTTP/1.1 200 OK
    < Date: Mon, 18 Jul 2016 06:22:24 GMT
    < Content-Type: text/html
    < Access-Control-Allow-Origin: *
    < Content-Length: 28
    <
    * Connection #0 to host localhost left intact
    "That is some good Mallory."

    OK: Mock server verified all requests, as expected

On OSX, the shared object would be `libpact_mock_server.dylib` and on windows `libpact_mock_server.dll`.
