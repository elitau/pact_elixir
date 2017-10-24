defmodule PactElixir.Dsl do
  alias PactElixir.{ServiceProvider, Interaction, Request, Response, PactMockServer, Errors}

  def service_provider(options) do
    %ServiceProvider{
      provider: options[:provider],
      consumer: options[:consumer],
      interactions: [],
      port: options[:port] || 0,
      pact_output_dir_path: options[:pact_output_dir_path]
    }
  end

  def build(provider) do
    provider
    |> ServiceProvider.to_pact_json
    |> PactMockServer.start(provider)
  end

  def mock_server_mismatches(provider) do
    PactMockServer.mismatches(provider) |> Poison.decode!
  end

  def mock_server_matched?(provider) do
    PactMockServer.matched?(provider)
  end

  # hook to run after each test
  def verify_interactions(provider) do
    provider
    |> mock_server_mismatches
    |> Errors.convert_to_errors
    |> throw_errors
  end

  def throw_errors([]) do
    # Logger.no_mismatches
  end

  def throw_errors(errors) do
    Enum.map(errors, &(raise(&1)))
  end

  # hook after test suite
  def after_test_suite(provider) do
    provider
    |> PactMockServer.write_pact_file
    # shutdown mock server
  end

  def add_interaction(provider, description, given, %Request{} = request, %Response{} = response) do
    interaction = %Interaction{
      description: description,
      given: given,
      request: request,
      response: response
    }
    # raise Pact::InvalidInteractionError.new(self) unless description && request && response
    put_in(provider.interactions, provider.interactions ++ [interaction])
  end

  def with_request(options) do
    %PactElixir.Request{
      method: options[:method] |> to_string |> String.upcase,
      path: options[:path]
    }
  end

  def will_respond_with(options) do
    %PactElixir.Response{
      status: options[:status],
      body: options[:body]
    }
  end

  def given(precondition) do
    precondition
  end
end
