defmodule PactElixir.Helpers do
  @moduledoc false
  defmacro __using__([]) do
    quote do
      import PactElixir.Dsl
      import unquote(__MODULE__)
    end
  end

  # Raise an error if called with an unknown framework
  #
  # defp setup_framework_integration(pact_elixir = %{pid: pid}) do
  #   ExUnit.Callbacks.on_exit({PactElixir, pid}, fn ->
  #     after_test_suite(pact_elixir.pid, ExUnit.AssertionError)
  #   end)
  # end

  defmacro pact_session(opts \\ []) do
    quote do
      setup do
        on_exit(fn -> after_test_suite(unquote(opts)) end)

        :ok
      end
    end
  end
end
