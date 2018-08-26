defmodule PactElixir.MixProject do
  @moduledoc false
  use Mix.Project

  def project do
    [
      app: :pact_elixir,
      version: "0.4.1",
      elixir: "~> 1.6",
      name: "PactElixir",
      start_permanent: Mix.env() == :prod,
      description: description(),
      package: package(),
      deps: deps(),
      test_coverage: [tool: ExCoveralls],
      preferred_cli_env: [
        coveralls: :test,
        "coveralls.detail": :test,
        "coveralls.post": :test,
        "coveralls.html": :test
      ],
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(),
      source_url: "https://github.com/elitau/pact_elixir",
      homepage_url: "https://github.com/elitau/pact_elixir",
      # The main page in the docs
      docs: [main: "readme", extras: ["README.md"]]
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
      {:ex_doc, "~> 0.18.0", only: :dev, runtime: false},
      {:httpoison, "~> 1.0", only: :test},
      {:excoveralls, "~> 0.7", only: :test},
      {:temp, "~> 0.4", only: :test},
      {:credo, "~> 0.8", only: [:dev, :test], runtime: false},
      {:inch_ex, "~> 1.0.0", only: :docs},
      {:dialyxir, "~> 0.5.1", only: [:dev, :test], runtime: false}
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

  defp description do
    """
    Elixir version of Pact. Enables consumer driven contract testing, providing a mock service and DSL for the consumer project.
    """

    # TODO Also provides interaction playback and verification for the service provider project.
  end

  defp package do
    [
      maintainers: ["Eduard Litau"],
      licenses: ["MIT"],
      files: ["lib", "native", "mix.exs", "README.md", "LICENSE"],
      links: %{"GitHub" => "https://github.com/elitau/pact_elixir"},
      source_url: "https://github.com/elitau/pact_elixir"
    ]
  end
end
