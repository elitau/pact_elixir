defmodule PactElixir.DslTest do
  use ExUnit.Case

  import PactElixir.Dsl

  test "Provider responds to /foo with 'bar'" do
    provider =
      service_provider(consumer: "PactTester", provider: "PactProvider")
      |> add_interaction(
        "give me foo",
        given("foo exists"),
        with_request(method: :get, path: "/foo"),
        will_respond_with(status: 200, body: "bar")
      )
      |> build

    assert get_request(provider, "/foo").body == "bar"

    assert "[]" = mock_server_mismatches(provider)
    assert mock_server_matched?(provider) == true

    {:ok}
  end

  test "Mock server includes mismatch without request" do
    provider =
      service_provider(consumer: "PactTester", provider: "PactProvider")
      |> add_interaction(
        "give me foo",
        given("foo exists"),
        with_request(method: :get, path: "/foo"),
        will_respond_with(status: 200, body: "bar")
      )
      |> build

    assert mock_server_matched?(provider) == false
    [failure | _tail] = mock_server_mismatches(provider) |> Poison.decode!
    assert %{"type" => "missing-request"} = failure

    {:ok}
  end

  defp get_request(provider, path) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{provider.port}#{path}")
  end
end
