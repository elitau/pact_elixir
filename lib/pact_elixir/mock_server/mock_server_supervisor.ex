defmodule PactElixir.MockServerSupervisor do
  @moduledoc """
  Supervises PactMockServers so that they can be controlled even
  after the test suite has finished. The Supervisor does not restart
  a failed PactMockServer as the information like successful
  requests is stored only there and therefor lost.
  """

  use DynamicSupervisor

  def start_link(_arg) do
    DynamicSupervisor.start_link(__MODULE__, :ok, name: __MODULE__)
  end

  def start_mock_server(%PactElixir.ServiceProvider{} = provider) do
    DynamicSupervisor.start_child(__MODULE__, {PactElixir.PactMockServer, provider})
  end

  def terminate_child(provider_pid) do
    case DynamicSupervisor.terminate_child(__MODULE__, provider_pid) do
      :ok -> :ok
      {:error, :not_found} -> :error
    end
  end

  def init(:ok) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end
end
