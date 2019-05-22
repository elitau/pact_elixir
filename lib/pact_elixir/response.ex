defmodule PactElixir.Response do
  @moduledoc """
  Represent the expected response.
  """
  # @derive [Poison.Encoder]
  defstruct [:body, :headers, :status]

  def new(attributes \\ %{}) do
    value_or_default = &value_from_map(attributes, &1, &2)

    %PactElixir.Response{
      body: value_or_default.(:body, "") |> collect_values_for_body(),
      headers: value_or_default.(:headers, %{}),
      status: value_or_default.(:status, 200)
    }
    |> PactElixir.TermDetector.recursively_update_terms()
  end

  defp value_from_map(attributes, name, default) do
    attributes[name] || attributes[:"#{name}"] || default
  end

  def collect_values_for_body(body) when is_map(body) do
    body
    |> Map.to_list()
    |> Enum.map(fn
      {k, %PactElixir.TypeMatcher{value: value}} -> {k, collect_values_for_body(value)}
      {k, %{} = v} -> {k, collect_values_for_body(v)}
      {k, v} -> {k, v}
    end)
    |> Enum.into(%{})
  end

  def collect_values_for_body(body) do
    body
  end

  def matching_rules(%__MODULE__{body: body}) do
  end
end

defimpl Poison.Encoder, for: PactElixir.Response do
  def encode(
        %PactElixir.Response{body: body, headers: headers, status: status} = response,
        options
      ) do
    Poison.Encoder.Map.encode(
      %{
        body: body,
        headers: headers,
        status: status,
        matchingRules: PactElixir.Response.matching_rules(response)
      },
      options
    )
  end
end
