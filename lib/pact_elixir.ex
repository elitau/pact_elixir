defmodule PactElixir do
  @moduledoc """
  Documentation for PactElixir.
  """

  def like(value) do
    PactElixir.TypeMatcher.like(value)
  end
end
