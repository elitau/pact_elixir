defmodule PactElixir.PactMockServerTest do
  use ExUnit.Case
  alias PactElixir.{PactMockServer, ServiceProvider}
  import PactElixir.Dsl

  setup do
    provider = new_service_provider()

    on_exit(fn ->
      delete_generated_pact_file(provider)
    end)

    {:ok, mock_server_pid} = start_supervised({PactMockServer, provider})
    {:ok, mock_server_pid: mock_server_pid, provider: provider}
  end

  describe "start mock server" do
    test "gen server is running", %{mock_server_pid: mock_server_pid} do
      assert Process.alive?(mock_server_pid)
    end

    test "registers process globally with name of provider" do
      provider = PactMockServer.service_provider("test_provider")
      assert "test_provider" = provider.provider
    end

    test "allows multiple mock servers" do
      another_provider = new_service_provider(%{provider: "another_provider"})

      assert {:ok, _pid} = PactMockServer.start_link(another_provider)
    end

    test "returns error if mock server for same provider is already running", %{
      provider: provider
    } do
      assert {:error, {:already_started, _pid}} =
               PactMockServer.start_link(new_service_provider(%{provider: provider.provider}))
    end

    # test "fails when starting on reserved port" do
    #   another_provider = new_service_provider(%{port: 22, provider: "reserved_port"})

    #   assert {:ok, _pid} = PactMockServer.start_link(another_provider)
    # end
  end

  test "return port of started mock server", %{mock_server_pid: mock_server_pid} do
    assert Enum.member?(1000..65_535, PactMockServer.port(mock_server_pid))
  end

  test "mock server responds to requests", %{mock_server_pid: mock_server_pid} do
    assert "bar" == do_example_request(mock_server_pid).body
  end

  # TODO: Simulate other possible errors during pact file write
  describe "write pact file" do
    test "file exists", %{mock_server_pid: mock_server_pid} do
      # make sure all assertions are matched which is needed for the file to be written
      do_example_request(mock_server_pid)

      assert {:ok} == PactMockServer.write_pact_file(mock_server_pid)
      assert File.exists?(PactMockServer.pact_file_path(mock_server_pid))
    end

    # matched? returns false if no server could be found for given port, so this test is somewhat misleading
    test "errors unless all assertions matched", %{mock_server_pid: mock_server_pid} do
      assert {:error, :mismatches_prohibited_file_output} ==
               PactMockServer.write_pact_file(mock_server_pid)

      refute File.exists?(PactMockServer.pact_file_path(mock_server_pid))
    end

    test "call with provider name", %{mock_server_pid: mock_server_pid} do
      # make sure all assertions are matched which is needed for the file to be written
      do_example_request(mock_server_pid)

      assert {:ok} == PactMockServer.write_pact_file("test_provider")
    end
  end

  describe "stop mock server" do
    test "shutdown GenServer", %{mock_server_pid: mock_server_pid} do
      assert PactMockServer.stop(mock_server_pid)
    end

    test "stopped mock server returns empty body", %{mock_server_pid: mock_server_pid} do
      port = PactMockServer.port(mock_server_pid)
      assert :ok == PactMockServer.stop(mock_server_pid)
      refute Process.alive?(mock_server_pid)
      assert "" == get_request("/foo", port).body
    end
  end

  describe "returns service provider" do
    test "as ServiceProvider struct", %{mock_server_pid: mock_server_pid} do
      assert %ServiceProvider{} = PactMockServer.service_provider(mock_server_pid)
    end

    test "as ServiceProvider struct for provider name" do
      assert %ServiceProvider{} = PactMockServer.service_provider("test_provider")
    end
  end

  def do_example_request(mock_server_pid) do
    get_request("/foo", mock_server_pid)
  end

  defp get_request(path, mock_server_pid) when is_pid(mock_server_pid) do
    get_request(path, PactMockServer.port(mock_server_pid))
  end

  defp get_request(path, port) when is_number(port) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{port}#{path}")
  end

  defp delete_generated_pact_file(provider) do
    exported_pact_file_path = ServiceProvider.pact_file_path(provider)
    if File.exists?(exported_pact_file_path), do: File.rm(exported_pact_file_path)
  end

  defp new_service_provider(options \\ %{}) do
    options
    |> service_provider()
    |> add_interaction(
      "give me foo",
      given("foo exists"),
      with_request(method: :get, path: "/foo"),
      will_respond_with(status: 200, body: "bar")
    )
  end
end
