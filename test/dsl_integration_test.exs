defmodule PactElixir.DslIntegrationTest do
  @moduledoc false
  use ExUnit.Case
  alias PactElixir.PactMockServer
  import PactElixir.Dsl

  setup context do
    provider = provider_with_interaction()

    unless context[:skip_after_test_suite_cleanup] do
      on_exit(fn -> shut_down_mock_server(provider) end)
    end

    exported_pact_file_path =
      Path.join(PactMockServer.pact_output_dir_path(provider), "PactTester-PactProvider.json")

    on_exit(fn ->
      if File.exists?(exported_pact_file_path) do
        File.rm(exported_pact_file_path)
      end
    end)

    {:ok, provider: provider}
  end

  test "Provider responds to /foo with 'bar'", %{provider: provider} do
    assert "\"bar\"" == get_request(provider, "/foo").body

    assert [] == mock_server_mismatches(provider)
    assert true == mock_server_matched?(provider)
  end

  test "has a host url", %{provider: provider} do
    port = PactElixir.PactMockServer.port(provider)
    assert mock_server_host_url(provider) == "http://localhost:#{port}"
  end

  test "Output all mismatches after single test finished" do
  end

  test "Output all mismatches after test suite finished" do
  end

  test "Tell that pact file was not written due to mismatches" do
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
    assert_raise PactElixir.VerificationError, fn ->
      verify_interactions(provider)
    end
  end

  @tag :skip_after_test_suite_cleanup
  test "write pact file if all interactions matched", %{provider: provider} do
    get_request(provider, "/foo")

    exported_pact_file_path =
      Path.join(PactMockServer.pact_output_dir_path(provider), "PactTester-PactProvider.json")

    assert :ok = after_test_suite(provider)

    assert File.exists?(exported_pact_file_path)
  end

  @tag :skip_after_test_suite_cleanup
  # This test has too much knowledge about the internals. Call it with provider name only
  test "genserver of mock server is killed after test suite", %{provider: provider} do
    get_request(provider, "/foo")
    pid = GenServer.whereis(PactMockServer.registered_name("PactProvider"))
    assert Process.alive?(pid)

    after_test_suite(provider)

    refute Process.alive?(pid)
  end

  @tag :skip
  test "convert all the errors" do
  end

  @tag :skip
  test "test from another project as real lib"

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
