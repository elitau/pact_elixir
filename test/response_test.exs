defmodule PactElixir.ResponseTest do
  use ExUnit.Case

  test "default constructor" do
    assert "" == PactElixir.Response.new().body
  end

  test "default constructor with arguments" do
    assert "foobar" == PactElixir.Response.new(%{body: "foobar"}).body
  end
end
