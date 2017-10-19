defmodule PactElixir.ServiceProvider do
  defstruct [:consumer, :provider, :port, :interactions]
end

defmodule PactElixir.Interaction do
  defstruct [:description, :given, :request, :response]
end

defimpl Poison.Encoder, for: PactElixir.Interaction do
  def encode(%{description: description, given: given, request: request, response: response}, options) do
    Poison.Encoder.Map.encode(
      %{
        description: description,
        providerState: given,
        request: request,
        response: response
      },
      options
    )
  end
end

defmodule PactElixir.Request do
  @derive [Poison.Encoder]
  defstruct [:method, :path]
end

defmodule PactElixir.Response do
  @derive [Poison.Encoder]
  defstruct [:status, :body]
end

defmodule PactElixir.Dsl do
  alias PactElixir.{ServiceProvider, Interaction, Request, Response}

  def service_provider(options) do
    %ServiceProvider{
      provider: options[:provider],
      consumer: options[:consumer],
      interactions: [],
      port: options[:port] || 0
    }
  end

  # returns Provider with actual port
  def start_mock_server(pact_json, %ServiceProvider{} = provider) do
    {:ok, mock_server_port} =
      PactElixir.PactMockServer.create_mock_server(pact_json, provider.port)

    put_in(provider.port, mock_server_port)
  end

  def build(provider) do
    provider
    |> to_pact_json
    |> start_mock_server(provider)
  end

  # defp verify_interactions(provider) do
    # get matches and mismatches
    # convert to exunit failures
    # output failures
    # write pact file
    # shutdown mock server
  # end

  def to_pact_json(provider) do
    """
      {
        "provider": {
          "name": "#{provider.provider}"
        },
        "consumer": {
          "name": "#{provider.consumer}"
        },
        "interactions": #{interactions_to_json(provider.interactions)},
        "metadata": {
          "pact-specification": {
            "version": "2.0.0"
          }
        }
      }
    """
  end

  def interactions_to_json(interactions) do
    Poison.encode!(interactions)
  end

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
