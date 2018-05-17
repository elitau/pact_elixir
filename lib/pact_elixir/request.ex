defmodule PactElixir.Request do
  @moduledoc """
  Represent the expected request.
  """
  @derive [Poison.Encoder]
  defstruct [:method, :path, :query, :headers, :body]

  def new(attributes \\ %{}) do
    value_or_default = &value_from_map(attributes, &1, &2)

    %PactElixir.Request{
      method: value_or_default.(:method, "GET") |> to_string |> String.upcase(),
      path: value_or_default.(:path, "/"),
      query: value_or_default.(:query, %{}) |> URI.encode_query(),
      headers: value_or_default.(:headers, %{}),
      body: value_or_default.(:body, "")
    }
  end

  defp value_from_map(attributes, name, default) do
    attributes[name] || attributes[:"#{name}"] || default
  end
end
