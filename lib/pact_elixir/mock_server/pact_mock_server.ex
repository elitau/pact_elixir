defmodule PactElixir.PactMockServer do
  @moduledoc """
  GenServer client API for managing a pact mock server.
  Used in conjunction with PactElixir.MockServerCallbacks.
  """
  alias PactElixir.ServiceProvider
  use GenServer

  # GenServer: Client
  def start_link(%ServiceProvider{} = provider) do
    GenServer.start_link(
      PactElixir.MockServerCallbacks,
      provider,
      name: registered_name(provider.provider)
    )
  end

  @spec port(pid) :: non_neg_integer
  def port(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:port})
  end

  @spec pact_output_dir_path(pid) :: String.t()
  def pact_output_dir_path(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:pact_output_dir_path})
  end

  @spec pact_file_path(pid) :: String.t()
  def pact_file_path(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:pact_file_path})
  end

  @spec mismatches(pid) :: String.t()
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

  def write_pact_file(name) when is_binary(name) do
    GenServer.call(registered_name(name), {:write_pact_file})
  end

  @spec service_provider(pid) :: PactElixir.ServiceProvider.t()
  def service_provider(name) when is_binary(name) do
    GenServer.call(registered_name(name), {:service_provider})
  end

  def service_provider(mock_server_pid) when is_pid(mock_server_pid) do
    GenServer.call(mock_server_pid, {:service_provider})
  end

  def registered_name(name) when is_binary(name) do
    # {:via, Registry, {PactElixir.Registry, name}}
    {:global, name}
  end

  def stop(mock_server_pid) when is_pid(mock_server_pid) do
    :ok = GenServer.stop(mock_server_pid)
  end

  # This is not used anywhere, just to shut up the warning elixir produces
  def init(_) do
    {:ok, nil}
  end
end
