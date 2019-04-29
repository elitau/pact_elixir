defmodule PactElixir.ResponseTest do
  use ExUnit.Case

  test "default constructor" do
    assert "" == PactElixir.Response.new().body
  end

  test "default constructor with arguments" do
    assert "foobar" == PactElixir.Response.new(%{body: "foobar"}).body
  end

  describe "with like matcher" do
    test "converts body attributes to usual attributes"

    test "converts body attributes to matching_rules" do
      assert %{} == PactElixir.Response.new(%{body: %{some: "value", with: PactElixir.like(23)}})
    end
  end
end
