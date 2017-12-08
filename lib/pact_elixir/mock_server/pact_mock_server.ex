defmodule PactElixir.PactMockServer do
  alias PactElixir.ServiceProvider
  use GenServer

  # GenServer: Client
  def start_link(%ServiceProvider{} = provider) do
    GenServer.start_link(PactElixir.MockServerCallbacks, provider)
  end

  def port(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:port})
  end

  def pact_output_dir_path(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:pact_output_dir_path})
  end

  def pact_file_path(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:pact_file_path})
  end

  def mismatches(mock_server_pid) when is_pid(mock_server_pid) do
    # TODO: fails with seg fault when called with not used port
    GenServer.call(mock_server_pid, {:mismatches})
  end

  @spec matched?(pid) :: list
  def matched?(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:matched})
  end

  def write_pact_file(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:write_pact_file})
  end

  def stop(mock_server_pid) when is_pid(mock_server_pid) do
    :ok = GenServer.stop(mock_server_pid)
  end
end