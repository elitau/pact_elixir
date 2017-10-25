defmodule PactElixir.Response do
  @derive [Poison.Encoder]
  defstruct [:status, :body]
end
