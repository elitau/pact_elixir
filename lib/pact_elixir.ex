defmodule PactElixir do
  @moduledoc """
  Documentation for PactElixir.
  """

  def like(value) do
    PactElixir.TypeMatcher.like(value)
  end

  def term(generate: generate, regex: regex) do
    PactElixir.Term.term(generate, regex)
  end

  def term(generate: generate, regexp: regexp) do
    PactElixir.Term.term(generate, regexp)
  end
end
