defmodule PactElixir.ServiceProvider do
  defstruct [:consumer, :provider, :port, :interactions]

  def to_pact_json(provider) do
    """
      {
        "provider": {
          "name": "#{provider.provider}"
        },
        "consumer": {
          "name": "#{provider.consumer}"
        },
        "interactions": #{PactElixir.Interaction.to_json(provider.interactions)},
        "metadata": {
          "pact-specification": {
            "version": "2.0.0"
          }
        }
      }
    """
  end
end
