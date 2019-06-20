defmodule PactElixir.ResponseTest do
  use ExUnit.Case

  test "default constructor" do
    assert "" == PactElixir.Response.new().body
  end

  test "default constructor with arguments" do
    assert "foobar" == PactElixir.Response.new(%{body: "foobar"}).body
  end

  describe "with like matcher" do
    test "converts body attributes to usual attributes" do
      assert %{body: %{some: "value"}} = PactElixir.Response.new(%{body: %{some: "value"}})
    end

    test "includes like matcher attributes as normal body attributes" do
      assert %{body: %{with: 23}} =
               PactElixir.Response.new(%{body: %{some: "value", with: PactElixir.like(23)}})
    end

    test "converts body attributes to matching_rules" do
      assert %{matching_rules: %{"$.body.with" => %{"match" => "type"}}} =
               PactElixir.Response.new(%{body: %{some: "value", with: PactElixir.like(23)}})
    end

    test "converts array attribute to matching_rule" do
      assert %{matching_rules: %{"$.body.with[0]" => %{"match" => "type"}}} =
               PactElixir.Response.new(%{body: %{some: "value", with: [PactElixir.like(23)]}})
    end
  end
end
