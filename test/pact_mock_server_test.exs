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

  test "creates a mock server and returns its port" do
    assert PactMockServer.create_mock_server(@pact, @port) == {:ok, @port}
    assert get_request("/call_me").body == "Stop calling me"
  end

  test "writes pact file" do
    provider = service_provider()
    PactMockServer.create_mock_server(@pact, @port)

    get_request("/call_me")

    assert PactMockServer.write_pact_file(provider) == {:success, true}
  end

  # TODO
  test "do not write pact file when no server is available on given port" do
    dir_path = Path.join(File.cwd!(), "test")
    exported_pact_file_path = Path.join(dir_path, "PactTester-PactProvider.json")

    assert {:error, :mismatches_prohibited_file_output} =
             PactMockServer.write_pact_file(%ServiceProvider{
               pact_output_dir_path: dir_path,
               port: 50456
             })

    refute File.exists?(exported_pact_file_path)
  end

  defp get_request(path) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{@port}#{path}")
  end

  defp service_provider(options \\ %{port: @port}) do
    PactElixir.Dsl.service_provider(options)
  end
end
