defmodule PactElixir.Examples.LikeMatcherTest do
  @moduledoc false
  use ExUnit.Case
  alias PactElixir.PactMockServer
  import PactElixir.Dsl

  setup do
    provider = provider_with_interaction()

    exported_pact_file_path =
      Path.join(PactMockServer.pact_output_dir_path(provider), "Consumer-Provider.json")

    on_exit(fn ->
      if File.exists?(exported_pact_file_path) do
        File.rm(exported_pact_file_path)
      end
    end)

    {:ok, provider: provider, exported_pact_file_path: exported_pact_file_path}
  end

  defp provider_with_interaction do
    pact_output_dir_path = Path.join(File.cwd!(), "test")

    service_provider(
      consumer: "Consumer",
      provider: "Provider",
      pact_output_dir_path: pact_output_dir_path
    )
    |> add_interaction(
      "a retrieve thing request",
      given("foo exists"),
      with_request(method: :get, path: "/thing"),
      will_respond_with(status: 200, body: %{company: PactElixir.like("FooBarString")})
    )
    |> build()
  end

  test "like matcher", %{provider: provider, exported_pact_file_path: exported_pact_file_path} do
    get_request(provider, "/thing")

    assert :ok = after_test_suite(provider)

    expected = File.read!("test/examples/like_matcher.pact.json") |> get_response_from_json()
    generated = File.read!(exported_pact_file_path) |> get_response_from_json()

    assert expected == generated
  end

  def get_response_from_json(response) do
    json = response |> Poison.decode!()
    [interation] = json["interactions"]
    interation["response"]
  end

  defp get_request(provider, path) do
    %HTTPoison.Response{} =
      HTTPoison.get!("http://localhost:#{PactMockServer.port(provider)}#{path}")
  end
end
