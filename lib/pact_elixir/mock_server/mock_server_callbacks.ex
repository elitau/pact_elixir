require Logger

defmodule PactElixir.MockServerCallbacks do
  @moduledoc """
  GenServer callbacks for managing a pact mock server.
  Used in conjunction with PactMockServer.
  """

  alias PactElixir.{ServiceProvider, RustPactMockServerFacade}

  def init(provider) do
    {:ok, start(provider)}
  end

  def handle_call({:port}, _from, provider) do
    {:reply, provider.port, provider}
  end

  def handle_call({:pact_output_dir_path}, _from, provider) do
    {:reply, provider.pact_output_dir_path, provider}
  end

  def handle_call({:pact_file_path}, _from, provider) do
    {:reply, ServiceProvider.pact_file_path(provider), provider}
  end

  def handle_call({:mismatches}, _from, provider) do
    {:reply, mismatches(provider), provider}
  end

  def handle_call({:matched}, _from, provider) do
    {:reply, matched?(provider), provider}
  end

  def handle_call({:write_pact_file}, _from, provider) do
    {:reply, write_pact_file(provider), provider}
  end

  def handle_call({:service_provider}, _from, provider) do
    {:reply, provider, provider}
  end

  def terminate(_reason, provider) do
    {:ok, _success} = RustPactMockServerFacade.cleanup_mock_server(provider.port)
  end

  # PactMockServer API
  # @spec start(ServiceProvider) :: ServiceProvider
  def start(%ServiceProvider{} = provider) do
    start(ServiceProvider.to_pact_json(provider), provider)
  end

  # returns ServiceProvider with actual port
  def start(pact_json, %ServiceProvider{} = provider) when is_binary(pact_json) do
    {:ok, mock_server_port} =
      RustPactMockServerFacade.create_mock_server(pact_json, provider.address)

    put_in(provider.port, mock_server_port)
  end

  def mismatches(%ServiceProvider{} = provider) do
    # TODO: fails with seg fault when called with not used port
    {:ok, mismatches} = RustPactMockServerFacade.mock_server_mismatches(provider.port)
    mismatches
  end

  # TODO: Dialyzer specs
  @spec matched?(ServiceProvider) :: boolean
  def matched?(%ServiceProvider{} = provider) do
    {:ok, matched} = RustPactMockServerFacade.mock_server_matched(provider.port)
    matched
  end

  def write_pact_file(%ServiceProvider{} = provider) do
    Logger.info("Writing Pact file to " <> PactElixir.ServiceProvider.pact_file_path(provider))
    write_pact_file_with_error_handling(provider, matched?(provider))
  end

  def write_pact_file_with_error_handling(%ServiceProvider{} = provider, true) do
    :ok = RustPactMockServerFacade.write_pact_file(provider.port, provider.pact_output_dir_path)
    {:ok}
  end

  def write_pact_file_with_error_handling(%ServiceProvider{} = _provider, false) do
    # Do not write file when mismatches happend
    {:error, :mismatches_prohibited_file_output}
  end
end
