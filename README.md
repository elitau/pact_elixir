# PactElixir

**This library is an Elixir wrapper for the [pact-reference](https://github.com/pact-foundation/pact-reference) implementation**

And it is in *PRE alpha* shape without any semantic versioning or documentation.

[![Build Status](https://travis-ci.org/elitau/pact_elixir.svg?branch=master)](https://travis-ci.org/elitau/pact_elixir)
[![Coverage Status](https://coveralls.io/repos/github/elitau/pact_elixir/badge.svg?branch=master)](https://coveralls.io/github/elitau/pact_elixir?branch=master)
[![Ebert](https://ebertapp.io/github/elitau/pact_elixir.svg)](https://ebertapp.io/github/elitau/pact_elixir)
[![Inline docs](http://inch-ci.org/github/elitau/pact_elixir.svg)](http://inch-ci.org/github/elitau/pact_elixir)

It is not yet usable as many needed parts like publishing a pact file to a broker or pact
verification on the provider side are still missing. Also there is no documentation available, yet.

## Installation

You need [Rust](https://www.rust-lang.org) in order to build and install the package.

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `pact_elixir` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:pact_elixir, "~> 0.5.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/pact_elixir](https://hexdocs.pm/pact_elixir).

## Examples

This is an example test case:

```elixir
defmodule PactElixir.PactMockServerTest do
  use ExUnit.Case
  alias PactElixir.{PactMockServer, ServiceProvider}
  import PactElixir.Dsl

  setup do
    options = %{provider: "SomeProvider", consumer: "SomeConsumer"}
    provider = new_service_provider(options)

    {:ok, mock_server_pid} = start_supervised({PactMockServer, provider})
    {:ok, mock_server_pid: mock_server_pid, provider: provider}
  end

  describe "SomeProvider talks to SomeConsumer" do
    test "some basic test", %{mock_server_pid: mock_server_pid} do
      expected = "{groups: ['Editors'], id: 123, username: 'UserA'}"

      user(mock_server_pid)

      assert expected == user(mock_server_pid).body
      assert {:ok} == PactMockServer.write_pact_file(mock_server_pid)
    end
  end

  defp get_request(path, mock_server_pid) when is_pid(mock_server_pid) do
    get_request(path, PactMockServer.port(mock_server_pid))
  end

  defp get_request(path, port) when is_number(port) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{port}#{path}")
  end

  def user(mock_server_pid) do
    get_request("/users/UserA", mock_server_pid)
  end

  defp new_service_provider(options \\ %{}) do
    options
    |> PactElixir.Dsl.service_provider()
    |> add_interaction(
      "give me foo",
      given("UserA exists and is not an administrator"),
      with_request(method: :get, path: "/users/UserA"),
      will_respond_with(status: 200, body: "{groups: ['Editors'], id: 123, username: 'UserA'}")
    )
  end
end
```

You should be able to run it with `mix test <path_to_test_case>`.

**Publishing**
If the test passes, a json file will be created and saved in a new directory - `./pacts`.

Currently, pact publishing is not yet implemented in this library. You can run the following bash script for your basic publishing needs:

```shell
#!/bin/bash
# This script:
# 1) extracts the name of provider and consumer from the name of a JSON pact file
# e.g.: Consumer1-Provider.json gives you consumer Consumer1 and provider Provider
# 2) reads the JSON pact file from /pacts directory (pacts are stored there by specification)
# 3) publishes pact to Pact Broker

for pact in ./pacts/*.json; do
    pact_name=$(basename $pact)

    consumer=${pact_name%-*}
    provider=${pact_name#*-}
    provider=${provider%.*}

    curl -v -XPUT \-H "Content-Type: application/json" \
    -d@${pact} \
    https://<your-pact-broker-url>/pacts/provider/$provider/consumer/$consumer/version/<some-version-number>
done
```

## Troubleshooting

```
Compiling NIF crate :pactmockserver (native/pactmockserver)...
could not compile dependency :pact_elixir, "mix compile"
failed. You can recompile this dependency with "mix deps.compile pact_elixir", update it with "mix deps.update pact_elixir" or clean it with "mix deps.clean pact_elixir"
```
This can be solved by ensuring proper structure of `mix.exs`.
1) Ensure you have `rustler` listed in compilers
2) Ensure `pactmockserver` is listed in rustler_crates

```elixir
def project do
    [
      #...
      compilers: [:phoenix, :rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(Mix.env()),
      #...
    ]
  end
```

(This is an example setting, where you don't want to ship any rustler crates to production. Bottom line is, there has to be a function that returns the crates list, including pactmockserver).

```elixir
defp rustler_crates(mix_env) when mix_env in [:test, :dev] do
    [
      pactmockserver: [
        path: "deps/pact_elixir/native/pactmockserver",
        mode: (:debug),
      ]
    ]
  end

  defp rustler_crates(_prod) do
    []
  end
  ```

### Docker
Properly installing Rust should solve a majority of the problems. Just add the following code (this is for a Debian distro, it assumes you have curl installed):

```docker
# Rust is required by pact-elixir
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=$HOME/.cargo/bin:$PATH
```
