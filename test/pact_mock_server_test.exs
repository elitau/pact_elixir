defmodule PactElixir.PactMockServerTest do
  use ExUnit.Case
  alias PactElixir.{PactMockServer, ServiceProvider}

  @pact """
  {
        "provider": {
          "name": "test_provider"
        },
        "consumer": {
          "name": "test_consumer"
        },
        "interactions": [
          {
            "providerState": "test state",
            "description": "test interaction",
            "request": {
              "method": "GET",
              "path": "/call_me"
            },
            "response": {
              "status": 200,
              "body": "Stop calling me"
            }
          }
        ],
        "metadata": {
          "pact-specification": {
            "version": "2.0.0"
          }
        }
      }
  """
  @port 50823

  setup do
    provider = ServiceProvider.new()

    on_exit(fn ->
      delete_generated_pact_file(provider)
      shutdown_mock_server(provider)
    end)

    {:ok, provider: provider}
  end

  test "creates a mock server and returns its port" do
    assert PactMockServer.create_mock_server(@pact, @port) == {:ok, @port}
    assert "Stop calling me" == get_request("/call_me").body
    PactMockServer.cleanup_mock_server(@port)
  end

  # todo
  # test "returns error if mock server port is already in use" do
  #   assert PactMockServer.create_mock_server(@pact, @port) == {:ok, @port}
  #   assert get_request("/call_me").body == "Stop calling me"
  #   PactMockServer.shutdown_mock_server(@port)
  # end

  test "writes pact file", %{provider: provider} do
    provider = PactMockServer.start(@pact, provider)

    # make sure all assertions are matched
    get_request("/call_me", provider.port)

    assert {:success, true} == PactMockServer.write_pact_file(provider)
  end

  # matched? returns false if no server could be found for given port, so this test is somewhat misleading
  test "do not write pact file when some assertions did not match", %{provider: provider} do
    assert {:error, :mismatches_prohibited_file_output} ==
             PactMockServer.write_pact_file(provider)

    refute File.exists?(ServiceProvider.pact_file_path(provider))
  end

  # TODO: Simulate other possible errors during pact file write

  test "shutdown mock server returns empty body", %{provider: provider} do
    provider = PactMockServer.start(@pact, provider)

    assert "Stop calling me" == get_request("/call_me", provider.port).body
    assert {:success, true} == shutdown_mock_server(provider)
    assert "" == get_request("/call_me", provider.port).body
  end

  def get_request(path, port \\ @port) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{port}#{path}")
  end

  defp delete_generated_pact_file(provider) do
    exported_pact_file_path = ServiceProvider.pact_file_path(provider)
    if File.exists?(exported_pact_file_path), do: File.rm(exported_pact_file_path)
  end

  defp shutdown_mock_server(provider) do
    PactMockServer.shutdown_mock_server(provider)
  end
end
