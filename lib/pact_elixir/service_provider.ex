defmodule PactElixir.ServiceProvider do
  @moduledoc """
  Represent the remote part (server/provider) for the expected interactions.
  """

  @type t :: %__MODULE__{
          consumer: String.t(),
          provider: String.t(),
          port: String.t() | non_neg_integer,
          address: String.t(),
          interactions: list(),
          pact_output_dir_path: String.t() | :none
        }

  defstruct [:consumer, :provider, :port, :address, :interactions, :pact_output_dir_path]
  @default_provider_name "test_provider"
  @default_consumer_name "test_consumer"
  # System selects a random port
  @default_port 0
  @default_address "localhost:#{@default_port}"
  @default_pact_output_dir_path "pacts/"

  def new(options \\ %{}) do
    %PactElixir.ServiceProvider{
      provider: options[:provider] || @default_provider_name,
      consumer: options[:consumer] || @default_consumer_name,
      interactions: [],
      address: options[:address] || @default_address,
      pact_output_dir_path: options[:pact_output_dir_path] || @default_pact_output_dir_path
    }
  end

  def pact_file_path(%__MODULE__{} = provider) do
    Path.join(provider.pact_output_dir_path, "#{provider.consumer}-#{provider.provider}.json")
  end

  def host_url(%PactElixir.ServiceProvider{port: port}) do
    "http://localhost:#{port}"
  end

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
          "pactSpecification": {
            "version": "2.0.0"
          }
        }
      }
    """
  end
end
