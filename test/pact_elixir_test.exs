defmodule PactElixirTest do
  use ExUnit.Case
  import PactElixir.Dsl

  test "Interactions to json" do
    expected_interactions_json =
      Poison.decode!("""
        [
          {
            "providerState": "foo is present",
            "description": "inter-description",
            "request": {
              "method": "GET",
              "path": "/foo"
            },
            "response": {
              "status": 200,
              "body": "bar"
            }
          }
        ]
      """)

    interaction = %PactElixir.Interaction{
      description: "inter-description",
      given: given("foo is present"),
      request: with_request(method: :get, path: "/foo"),
      response: will_respond_with(status: 200, body: "bar")
    }

    actual_json = PactElixir.Interaction.to_json([interaction]) |> Poison.decode!()
    assert expected_interactions_json == actual_json
  end
end
