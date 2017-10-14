defmodule PactElixir.PactMockServer do
    use Rustler, otp_app: :pact_elixir, crate: "pactmockserver"

    # When your NIF is loaded, it will override this functions.

    @doc """
    Add numbers

    ## Examples

        iex> PactElixir.PactMockServer.add(2, 3)
        {:ok, 5}

    """
    def add(_a, _b), do: throw :nif_not_loaded

    # Create a mock server
    def create_mock_server(_pact_json, _port), do: throw :nif_not_loaded
end