defmodule PactElixir.MixProject do
  use Mix.Project

  def project do
    [
      app: :pact_elixir,
      version: "0.1.0",
      elixir: "~> 1.6-dev",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {PactElixir.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.10"},
      {:poison, "~> 3.1"},
      {:httpoison, "~> 0.13", only: :test}
    ]
  end

  def rustler_crates do
    [
      pactmockserver: [
        path: "native/pactmockserver",
        mode: if(Mix.env() == :prod, do: :release, else: :debug)
      ]
    ]
  end
end
