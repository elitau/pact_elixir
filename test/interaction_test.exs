defmodule PactElixir.InteractionTest do
  use ExUnit.Case
  import PactElixir.Dsl

  test "Interactions to json" do
    expected_interactions_json =
      Poison.decode!("""
        [
          {
            "providerState": "foo is present",
            "description": "interaction-description",
            "request": {
              "method": "GET",
              "path": "/foo",
              "query": "sort=desc",
              "headers": {},
              "body": ""
            },
            "response": {
              "status": 200,
              "body": "bar",
              "headers": {}
            }
          }
        ]
      """)

    interaction = %PactElixir.Interaction{
      description: "interaction-description",
      given: given("foo is present"),
      request: with_request(method: :get, path: "/foo", query: %{sort: "desc"}),
      response: will_respond_with(status: 200, body: "bar")
    }

    actual_json =
      [interaction]
      |> PactElixir.Interaction.to_json()
      |> Poison.decode!()

    assert expected_interactions_json == actual_json
  end
end
