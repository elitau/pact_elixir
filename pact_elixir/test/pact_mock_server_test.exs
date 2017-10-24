defmodule PactElixir.PactMockServerTest do
  use ExUnit.Case
  alias PactElixir.PactMockServer

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
          },
          "pact-jvm": {
            "version": ""
          }
        }
      }
  """
  @port 50823

  test "creates a mock server and returns its port" do
    assert PactMockServer.create_mock_server(@pact, @port) == {:ok, @port}
    assert get_request("/call_me").body == "Stop calling me"
  end

  # test "do not write pact file when mismatches happend" do
  #   dir_path = Path.join(File.cwd!, "test")
  #   exported_pact_file_path = Path.join(dir_path, "PactTester-PactProvider.json")
  #   PactMockServer.create_mock_server(@pact, @port) == {:ok, @port}

  #   assert {:ok, result} = PactMockServer.write_pact_file(@port, dir_path)

  #   refute File.exists?(exported_pact_file_path)
  # end

  defp get_request(path) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{@port}#{path}")
  end
end
