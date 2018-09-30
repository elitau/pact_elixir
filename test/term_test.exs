defmodule PactElixir.TermTest do
  alias PactElixir.Term, as: Term
  use ExUnit.Case

  test "default constructor with arguments" do
    new_term = %Term{generate: "hey", regex: "some RegEx"}
    assert "some RegEx" == new_term.regex
  end

  test "default constructor without arguments" do
    new_term = %Term{}
    assert nil == new_term.regex
  end
end
