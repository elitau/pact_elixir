defmodule PactElixir.DslTest do
  use ExUnit.Case

  import PactElixir.Dsl

  test "Provider responds to /foo with 'bar'" do
    provider = provider_with_interaction()

    assert get_request(provider, "/foo").body == "bar"

    assert mock_server_mismatches(provider) == []
    assert mock_server_matched?(provider) == true
  end

  test "Mock server includes mismatch without mocked request being made" do
    provider = provider_with_interaction()

    assert mock_server_matched?(provider) == false
    [failure | _tail] = mock_server_mismatches(provider)
    assert %{
              "method" => "GET",
              "path" => "/foo",
              "type" => "missing-request"
            } = failure
  end

  test "fails with ex unit assertion error" do
    provider = provider_with_interaction()

    assert_raise PactElixir.RequestError, fn ->
      provider
      |> verify_interactions
    end
  end

  # test "throws InvalidInteractionError when description is missing"
  # test "throws InvalidInteractionError when request is missing"
  # test "throws InvalidInteractionError when response is missing"

  defp get_request(provider, path) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{provider.port}#{path}")
  end

  defp provider_with_interaction do
    service_provider(consumer: "PactTester", provider: "PactProvider")
    |> add_interaction(
      "give me foo",
      given("foo exists"),
      with_request(method: :get, path: "/foo"),
      will_respond_with(status: 200, body: "bar")
    )
    |> build
  end
end
