defmodule PactElixir.MockServerSupervisor do
  use DynamicSupervisor

  def start_link(_arg) do
    DynamicSupervisor.start_link(__MODULE__, :ok, name: __MODULE__)
  end

  def start_mock_server(%PactElixir.ServiceProvider{} = provider) do
    opts = %{
      id: PactElixir.PactMockServer,
      start: {PactElixir.PactMockServer, :start_link, [provider]}
    }

    DynamicSupervisor.start_child(PactElixir.MockServerSupervisor, opts)
  end

  def terminate_child(provider_pid) do
    DynamicSupervisor.terminate_child(PactElixir.MockServerSupervisor, provider_pid)
  end

  def init(:ok) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end
end
