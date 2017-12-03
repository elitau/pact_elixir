defmodule PactElixir.PactMockServerTest do
  use ExUnit.Case
  alias PactElixir.{PactMockServer, ServiceProvider}
  import PactElixir.Dsl

  setup do
    provider =
      service_provider()
      |> add_interaction(
           "give me foo",
           given("foo exists"),
           with_request(method: :get, path: "/foo"),
           will_respond_with(status: 200, body: "bar")
         )

    on_exit(fn ->
      delete_generated_pact_file(provider)
    end)

    {:ok, provider: provider}
  end

  # todo
  # test "returns error if mock server port is already in use" do
  #   assert PactMockServer.create_mock_server(@pact, @port) == {:ok, @port}
  #   assert get_request("/call_me").body == "Stop calling me"
  #   PactMockServer.shutdown_mock_server(@port)
  # end

  test "return port of started mock server", %{provider: provider} do
    {:ok, mock_server_pid} = start_supervised({PactMockServer, provider})
    IO.inspect(PactMockServer.port(mock_server_pid))
    assert Enum.member?(40000..65535, PactMockServer.port(mock_server_pid))
  end

  test "spawn as genserver", %{provider: provider} do
    {:ok, mock_server_pid} = start_supervised({PactMockServer, provider})

    assert "bar" == get_request("/foo", mock_server_pid).body
  end

  test "shutdown GenServer", %{provider: provider} do
    {:ok, mock_server_pid} = start_supervised({PactMockServer, provider})

    assert PactMockServer.stop(mock_server_pid)
  end

  test "writes pact file", %{provider: provider} do
    {:ok, mock_server_pid} = start_supervised({PactMockServer, provider})

    # make sure all assertions are matched which is needed for the file to be written
    get_request("/foo", mock_server_pid)

    assert {:success, true} == PactMockServer.write_pact_file(mock_server_pid)
  end

  # matched? returns false if no server could be found for given port, so this test is somewhat misleading
  test "do not write pact file when some assertions did not match", %{provider: provider} do
    {:ok, mock_server_pid} = start_supervised({PactMockServer, provider})

    assert {:error, :mismatches_prohibited_file_output} ==
             PactMockServer.write_pact_file(mock_server_pid)

    refute File.exists?(PactMockServer.pact_file_path(mock_server_pid))
  end

  # TODO: Simulate other possible errors during pact file write

  test "shutdown mock server returns empty body", %{provider: provider} do
    {:ok, mock_server_pid} = start_supervised({PactMockServer, provider})

    assert "bar" == get_request("/foo", mock_server_pid).body
    port = PactMockServer.port(mock_server_pid)
    assert :ok == PactMockServer.stop(mock_server_pid)
    refute Process.alive?(mock_server_pid)
    assert "" == get_request("/foo", port).body
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
end