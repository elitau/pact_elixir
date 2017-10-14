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
              "method": "POST",
              "path": "/",
              "body": {
                "complete": {
                  "certificateUri": "http://...",
                  "issues": {
                    "idNotFound": {}
                  },
                  "nevdis": {
                    "body": null,
                    "colour": null,
                    "engine": null
                  },
                  "body": 123456
                },
                "body": [
                  1,
                  2,
                  3
                ]
              }
            },
            "response": {
              "status": 200
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

  test "creates a mock server and returns its port" do
    assert PactMockServer.create_mock_server(@pact, 50823) == {:ok, 50823}
  end
end
