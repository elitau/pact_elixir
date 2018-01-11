# PactElixir

**This library is an Elixir wrapper for the [pact-reference](https://github.com/pact-foundation/pact-reference) implementation**

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
    {:pact_elixir, "~> 0.1.1"}
  ]
end
```
=>

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/pact_elixir](https://hexdocs.pm/pact_elixir).
