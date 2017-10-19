defmodule PactElixir.Dsl do
  alias PactElixir.{ServiceProvider, Interaction, Request, Response, MockServer}

  def service_provider(options) do
    %ServiceProvider{
      provider: options[:provider],
      consumer: options[:consumer],
      interactions: [],
      port: options[:port] || 0
    }
  end

  def build(provider) do
    provider
    |> ServiceProvider.to_pact_json
    |> MockServer.start_mock_server(provider)
  end

  # defp verify_interactions(provider) do
    # get matches and mismatches
    # convert to exunit failures
    # output failures
    # write pact file
    # shutdown mock server
  # end

  def add_interaction(provider, description, given, %Request{} = request, %Response{} = response) do
    interaction = %Interaction{
      description: description,
      given: given,
      request: request,
      response: response
    }

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
