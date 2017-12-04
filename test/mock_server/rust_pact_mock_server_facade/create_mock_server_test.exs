defmodule PactElixir.RustPactMockServerFacadeTest do
  use ExUnit.Case
  alias PactElixir.RustPactMockServerFacade

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
    assert {:ok, @port} == RustPactMockServerFacade.create_mock_server(@pact, @port)
    assert "Stop calling me" == get_request("/call_me").body
    RustPactMockServerFacade.cleanup_mock_server(@port)
  end

  test "fails if mock server could not start due to broken json" do
    broken_json = "broken{}json"

    assert {:error, error_message} =
             RustPactMockServerFacade.create_mock_server(broken_json, @port)

    assert "InvalidPactJson" == error_message
    RustPactMockServerFacade.cleanup_mock_server(@port)
  end

  # test "fails if mock server could not start because of other error" do
  #   assert {:error, error_message} = RustPactMockServerFacade.create_mock_server(@pact, 22)

  #   assert "MockServerFailedToStart" == error_message
  #   RustPactMockServerFacade.cleanup_mock_server(@port)
  # end

  def get_request(path) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{@port}#{path}")
  end
end