defmodule PactElixir.Interaction do
  defstruct [:description, :given, :request, :response]

  def to_json(interactions) do
    Poison.encode!(interactions)
  end
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
