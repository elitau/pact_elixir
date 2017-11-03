defmodule PactElixir.RequestTest do
  use ExUnit.Case

  test "default constructor" do
    assert "GET" == PactElixir.Request.new().method
  end

  test "default constructor with arguments" do
    assert "POST" == PactElixir.Request.new(%{method: "post"}).method
  end
end
