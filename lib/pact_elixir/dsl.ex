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

  # hook to run after each test
  def verify_interactions(provider) do
    provider
    |> mock_server_mismatches
    |> Errors.convert_to_errors()
    |> throw_errors
  end

  def throw_errors([]) do
    # Logger.no_mismatches
  end

  def throw_errors(errors) do
    Enum.map(errors, &raise(&1))
  end

  def after_test_suite(provider_name) when is_binary(provider_name) do
    provider_name
    |> PactMockServer.registered_name()
    |> GenServer.whereis()
    |> after_test_suite

    # shutdown mock server
  end

  def after_test_suite(provider_pid) when is_pid(provider_pid) do
    PactMockServer.write_pact_file(provider_pid)
  end

  def after_test_suite(providers) when is_list(providers) do
    Enum.map(providers, &after_test_suite/1)
  end

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
