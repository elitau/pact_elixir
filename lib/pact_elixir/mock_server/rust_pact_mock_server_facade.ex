defmodule PactElixir.RustPactMockServerFacade do
  use Rustler, otp_app: :pact_elixir, crate: "pactmockserver"

  def create_mock_server(_pact_json, _port), do: throw(:nif_not_loaded)
  def mock_server_mismatches(_port), do: throw(:nif_not_loaded)

  # @spec mock_server_matched(number) :: {:ok, boolean}
  def mock_server_matched(_port), do: throw(:nif_not_loaded)
  def write_pact_file(_port, _dir_path), do: throw(:nif_not_loaded)
  def cleanup_mock_server(_port), do: throw(:nif_not_loaded)
end