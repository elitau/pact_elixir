defmodule PactElixir.RustPactMockServerFacade do
  @moduledoc """
  Adapter for the wrapped rust [pact mock server](https://github.com/pact-foundation/pact-reference).
  Functions in this file are replaced by Rustler with their Rust calling
  counterpart. See native/pactmockserver/src/lib.rs for the concrete Rust
  implementation.
  This file is excluded from the coverage tool.
  """

  require Logger

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  rustler_opts = [otp_app: :pact_elixir, crate: "pactmockserver", mode: :release]
  env_config = Application.get_env(rustler_opts[:otp_app], PactElixir, [])

  opts =
    if System.get_env("PACTELIXIR_BUILD") in ["1", "true"] or env_config[:build_from_source] do
      rustler_opts
    else
      case PactElixir.RustPactMockServerFacade.Precompiled.download_or_reuse_nif_file(
             rustler_opts,
             base_url: "#{github_url}/releases/download/v#{version}",
             version: version
           ) do
        {:ok, new_opts} ->
          new_opts

        {:error, error} ->
          error =
            "Error while downloading precompiled NIF: #{error}\n\nSet HTML5EVER_BUILD=1 env var to compile the NIF from scratch. You can also configure this application to force compilation:\n\n    config :html5ever, Html5ever, build_from_source: true\n"

          if Mix.env() == :prod do
            raise error
          else
            Logger.debug(error)
            rustler_opts
          end
      end
    end

  use Rustler, opts

  def create_mock_server(_pact_json, _port), do: throw(:nif_not_loaded)
  def mock_server_mismatches(_port), do: throw(:nif_not_loaded)

  # @spec mock_server_matched(number) :: {:ok, boolean}
  def mock_server_matched(_port), do: throw(:nif_not_loaded)
  def write_pact_file(_port, _dir_path), do: throw(:nif_not_loaded)
  def cleanup_mock_server(_port), do: throw(:nif_not_loaded)
end
