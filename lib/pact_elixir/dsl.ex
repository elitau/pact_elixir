defmodule PactElixir.Dsl do
  @moduledoc """
  This module can be imported to create and start a mock server with the usual Pact DSL.
  """
  alias PactElixir.{ServiceProvider, Interaction, Request, Response, PactMockServer, Errors}

  def service_provider(options \\ %{}) do
    ServiceProvider.new(options)
  end

  def build(provider) do
    {:ok, pid} = PactElixir.MockServerSupervisor.start_mock_server(provider)
    pid
  end

  # TODO: capture source location of request/response interaction definition for error output
  @doc """
  Adds an interaction to the pact.
  """
  def add_interaction(provider, description, given, %Request{} = request, %Response{} = response) do
    interaction = %Interaction{
      description: description,
      given: given,
      request: request,
      response: response
    }

    # TODO: raise Pact::InvalidInteractionError.new(self) unless description && request && response
    put_in(provider.interactions, provider.interactions ++ [interaction])
  end

  def with_request(options) do
    PactElixir.Request.new(options)
  end

  def will_respond_with(options) do
    PactElixir.Response.new(options)
  end

  def given(precondition) do
    precondition
  end

  # @deprecated "Should only be used internally" Why, oh why, is this here?
  def mock_server_mismatches(provider) when is_pid(provider) do
    provider
    |> PactMockServer.mismatches()
    |> Poison.decode!()
  end

  # @deprecated "Should only be used internally" Why, oh why, is this here?
  def mock_server_matched?(provider) do
    PactMockServer.matched?(provider)
  end

  @doc """
  Returns URL of the mock server host, eg. "http://localhost:54321"
  """
  def mock_server_host_url(provider) do
    provider
    |> PactMockServer.service_provider()
    |> ServiceProvider.host_url()
  end

  @doc """
  Verifies interactions and shuts the mock server down.
  To be called after test suite run in main application, eg. in an on_exit callback:
  on_exit(fn ->
    after_test_suite(provider)
  end)
  """
  @spec after_test_suite(pid | String.t()) :: :ok | :error
  def after_test_suite(provider_pid) when is_pid(provider_pid) do
    verify_interactions(provider_pid)
    write_pact_file(provider_pid)
    shut_down_mock_server(provider_pid)
    :ok
  end

  def after_test_suite(provider_name) when is_binary(provider_name) do
    provider_name
    |> PactMockServer.registered_name()
    |> GenServer.whereis()
    |> after_test_suite
  end

  # @deprecated "Should only be used internally" Why, oh why, is this here?
  @spec shut_down_mock_server(pid) :: :ok | :error
  def shut_down_mock_server(provider_pid) when is_pid(provider_pid) do
    PactElixir.MockServerSupervisor.terminate_child(provider_pid)
  end

  # Checks whether all expectations were met
  # @deprecated "Should only be used internally" Why, oh why, is this here?
  @spec verify_interactions(pid) :: :ok
  def verify_interactions(provider_pid) when is_pid(provider_pid) do
    provider_pid
    |> mock_server_mismatches
    |> Errors.convert_to_error()
  end

  defp write_pact_file(provider_pid) do
    PactMockServer.write_pact_file(provider_pid)
  end
end
