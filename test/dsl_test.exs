defmodule PactElixir.DslTest do
  use ExUnit.Case
  alias PactElixir.PactMockServer
  import PactElixir.Dsl

  # "setup" is called before each test
  setup do
    {:ok, provider: provider_with_interaction()}
  end

  test "Provider responds to /foo with 'bar'", %{provider: provider} do
    assert "bar" == get_request(provider, "/foo").body

    assert [] == mock_server_mismatches(provider)
    assert true == mock_server_matched?(provider)
  end

  test "Mock server includes mismatch without mocked request being made", %{provider: provider} do
    assert mock_server_matched?(provider) == false

    [failure | _tail] = mock_server_mismatches(provider)

    assert %{
             "method" => "GET",
             "path" => "/foo",
             "type" => "missing-request"
           } = failure
  end

  test "fails with ex unit assertion error", %{provider: provider} do
    assert_raise PactElixir.RequestError, fn ->
      provider
      |> verify_interactions
    end
  end

  test "write pact file after test suite", %{provider: provider} do
    exported_pact_file_path =
      Path.join(PactMockServer.pact_output_dir_path(provider), "PactTester-PactProvider.json")

    on_exit(fn ->
      if File.exists?(exported_pact_file_path) do
        File.rm(exported_pact_file_path)
      end
    end)

    get_request(provider, "/foo")

    after_test_suite(provider)

    assert File.exists?(exported_pact_file_path)
  end

  # test "throws InvalidInteractionError when description is missing"
  # test "throws InvalidInteractionError when request is missing"
  # test "throws InvalidInteractionError when response is missing"

  defp get_request(provider, path) do
    %HTTPoison.Response{} =
      HTTPoison.get!("http://localhost:#{PactMockServer.port(provider)}#{path}")
  end

  defp provider_with_interaction do
    pact_output_dir_path = Path.join(File.cwd!(), "test")

    service_provider(
      consumer: "PactTester",
      provider: "PactProvider",
      pact_output_dir_path: pact_output_dir_path
    )
    |> add_interaction(
         "give me foo",
         given("foo exists"),
         with_request(method: :get, path: "/foo"),
         will_respond_with(status: 200, body: "bar")
       )
    |> build
  end
end