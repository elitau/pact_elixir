defmodule PactElixir.MockServerCallbacks do
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

  def terminate(reason, provider) do
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
      RustPactMockServerFacade.create_mock_server(pact_json, provider.port)

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
    write_pact_file_with_error_handling(provider, matched?(provider))
  end

  defp write_pact_file_with_error_handling(%ServiceProvider{} = provider, true) do
    RustPactMockServerFacade.write_pact_file(provider.port, provider.pact_output_dir_path)
    |> process_write_pact_file_error
  end

  defp write_pact_file_with_error_handling(%ServiceProvider{} = _provider, false) do
    # Do not write file when mismatches happend
    {:error, :mismatches_prohibited_file_output}
  end

  # Successfully written
  defp process_write_pact_file_error({:ok, 0}), do: {:success, true}
  defp process_write_pact_file_error({:ok, 1}), do: {:error, :general_panic_caught}

  defp process_write_pact_file_error({:ok, 2}),
    do: {:error, :pact_file_was_not_able_to_be_written}

  defp process_write_pact_file_error({:ok, 3}),
    do: {:error, :mock_server_with_the_provided_port_was_not_found}
end