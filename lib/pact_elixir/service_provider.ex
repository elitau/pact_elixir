defmodule PactElixir.ServiceProvider do
  defstruct [:consumer, :provider, :port, :interactions, :pact_output_dir_path]
  @default_provider_name "test_provider"
  @default_consumer_name "test_consumer"
  # System selects a random port
  @default_port 0
  @default_pact_output_dir_path File.cwd!()

  def new(options) do
    %PactElixir.ServiceProvider{
      provider: options[:provider] || @default_provider_name,
      consumer: options[:consumer] || @default_consumer_name,
      interactions: [],
      port: options[:port] || @default_port,
      pact_output_dir_path: options[:pact_output_dir_path] || @default_pact_output_dir_path
    }
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
          "pact-specification": {
            "version": "2.0.0"
          }
        }
      }
    """
  end
end
