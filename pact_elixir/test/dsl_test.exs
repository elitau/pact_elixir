defmodule PactElixir.DslTest do
  use ExUnit.Case

  import PactElixir.Dsl

  # setup do
  #   # on_exit do
  #   #   # failures = verify_interactions(provider)
  #   #   :ok
  #   # end
  # end

  test "Provider responds to /foo with 'bar'" do
    provider =
      service_provider(consumer: "PactTester", provider: "PactProvider")
      |> add_interaction(
        "give me foo",
        given("foo exists"),
        with_request(method: :get, path: "/foo"),
        will_respond_with(status: 200, body: "bar")
      )
      |> add_interaction(
        "give me foo",
        given("foo exists"),
        with_request(method: :get, path: "/foo"),
        will_respond_with(status: 200, body: "bar")
      )
      |> build

    assert get_request(provider, "/foo").body == "bar"
    {:ok}
  end

  test "Interactions to json" do
    expected_interactions_json = Poison.decode! """
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
    """
    interaction = %PactElixir.Interaction{
      description: "inter-description",
      given: given("foo is present"),
      request: with_request(method: :get, path: "/foo"),
      response: will_respond_with(status: 200, body: "bar")
    }
    actual_json = interactions_to_json([interaction]) |> Poison.decode!
    assert expected_interactions_json == actual_json
  end

  defp get_request(provider, path) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{provider.port}#{path}")
  end
end
