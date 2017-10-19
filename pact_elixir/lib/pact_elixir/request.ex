defmodule PactElixir.Request do
  @derive [Poison.Encoder]
  defstruct [:method, :path]
end
