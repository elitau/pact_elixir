defmodule PactElixir.ServiceProviderTest do
  use ExUnit.Case
  alias PactElixir.ServiceProvider

  test "default constructor values" do
    provider = ServiceProvider.new()

    assert 0 = provider.port
    assert "pacts/" = provider.pact_output_dir_path
  end

  test "constructor parameters" do
    provider = ServiceProvider.new(port: 52_342)

    assert 52_342 == provider.port
  end

  test "custom name for service provider" do
    provider = ServiceProvider.new(provider: "CustomProviderName")

    assert "CustomProviderName" = provider.provider
  end
end
