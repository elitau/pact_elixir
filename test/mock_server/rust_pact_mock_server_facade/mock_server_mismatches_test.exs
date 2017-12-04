defmodule PactElixir.RustPactMockServerFacade.MockServerMismatchesTest do
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
  @port 50824

  test "returns mismatches json when no requests were made" do
    RustPactMockServerFacade.create_mock_server(@pact, @port)
    assert {:ok, mismatches_json_string} = RustPactMockServerFacade.mock_server_mismatches(@port)

    assert String.ends_with?(mismatches_json_string, "}]")
    RustPactMockServerFacade.cleanup_mock_server(@port)
  end
end