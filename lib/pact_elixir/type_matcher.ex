defmodule PactElixir.TypeMatcher do
  @moduledoc """
  Allow to match against a type instead of an exact value.
  """
  defstruct [:value]

  def like(value) do
    %__MODULE__{value: value}
  end
end
