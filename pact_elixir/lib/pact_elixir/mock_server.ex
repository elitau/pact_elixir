defmodule PactElixir.MockServer do
  alias PactElixir.ServiceProvider

  # returns ServiceProvider with actual port
  def start_mock_server(pact_json, %ServiceProvider{} = provider) do
    {:ok, mock_server_port} =
      PactElixir.PactMockServer.create_mock_server(pact_json, provider.port)

    put_in(provider.port, mock_server_port)
  end
end