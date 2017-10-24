defmodule PactElixir.PactMockServer do
  use Rustler, otp_app: :pact_elixir, crate: "pactmockserver"
  # When your NIF is loaded, it will override this functions.

  @doc """
  Add numbers

  ## Examples

      iex> PactElixir.PactMockServer.add(2, 3)
      {:ok, 5}

  """
  def add(_a, _b), do: throw(:nif_not_loaded)

  alias PactElixir.ServiceProvider

  # returns ServiceProvider with actual port
  def start(pact_json, %ServiceProvider{} = provider) do
    {:ok, mock_server_port} =
      create_mock_server(pact_json, provider.port)

    put_in(provider.port, mock_server_port)
  end

  # Create a mock server
  def create_mock_server(_pact_json, _port), do: throw(:nif_not_loaded)

  def mismatches(%ServiceProvider{} = provider) do
    {:ok, mismatches} = mock_server_mismatches(provider.port)
    mismatches
  end

  def mock_server_mismatches(_port), do: throw(:nif_not_loaded)

  def matched?(%ServiceProvider{} = provider) do
    {:ok, matched} = mock_server_matched(provider.port)
    matched
  end

  def mock_server_matched(_port), do: throw(:nif_not_loaded)

  def write_pact_file(%ServiceProvider{} = provider) do
    result = if matched?(provider) do
      write_pact_file(provider.port, provider.pact_output_dir_path)
      |> process_write_pact_file_error
    else
      # Do not write file when mismatches happend
      {:mismatches_happend, false}
    end

    result
  end

  # Successfully written
  defp process_write_pact_file_error({:ok, 0}), do: true
  # TODO: raise error unless result == 0
  # Errors are returned as positive values.
  #
  # | Error | Description |
  # |-------|-------------|
  # | 1 | A general panic was caught |
  # | 2 | The pact file was not able to be written |
  # | 3 | A mock server with the provided port was not found |

  def write_pact_file(_port, _dir_path), do: throw(:nif_not_loaded)
end
