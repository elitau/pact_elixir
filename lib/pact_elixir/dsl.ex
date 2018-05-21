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

  def mock_server_mismatches(provider) when is_pid(provider) do
    provider
    |> PactMockServer.mismatches()
    |> Poison.decode!()
  end

  def mock_server_matched?(provider) do
    PactMockServer.matched?(provider)
  end

  # raises an error for existing mismatches
  def report_errors(provider) do
    provider
    |> mock_server_mismatches
    |> Errors.convert_to_error()
  end

  def after_test_suite(provider_name) when is_binary(provider_name) do
    provider_name
    |> PactMockServer.registered_name()
    |> GenServer.whereis()
    |> after_test_suite
  end

  # Call after successful test suite run
  def after_test_suite(provider_pid) when is_pid(provider_pid) do
    PactElixir.MockServerSupervisor.terminate_child(provider_pid)
  end

  def verify_pact(provider_pid) do
    provider_pid
    |> verify_interactions
    |> after_test_suite
  end

  # Checks whether all expectations were met
  def verify_interactions(provider_pid) when is_pid(provider_pid) do
    case mock_server_matched?(provider_pid) do
      true -> PactMockServer.write_pact_file(provider_pid)
      _ -> provider_pid |> report_errors()
    end

    provider_pid
  end

  # def after_test_suite(providers) when is_list(providers) do
  #   Enum.map(providers, &after_test_suite/1)
  # end

  # todo: capture source location of request/response interaction definition for error output
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
end
