defmodule PactElixir.Response do
  @moduledoc """
  Represent the expected response.
  """
  # @derive [Poison.Encoder]
  defstruct [:body, :headers, :status, :matching_rules]

  def new(attributes \\ %{}) do
    value_or_default = &value_from_map(attributes, &1, &2)

    %PactElixir.Response{
      body: value_or_default.(:body, "") |> collect_values_for_body(),
      headers: value_or_default.(:headers, %{}),
      status: value_or_default.(:status, 200),
      matching_rules: value_or_default.(:body, %{}) |> matching_rules()
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

  def matching_rules(body), do: do_matching_rules(body, [:"$", :body], %{})

  def do_matching_rules(content, current_path, rules) when is_map(content) do
    content
    |> Enum.reduce(rules, fn
      {key, %PactElixir.TypeMatcher{value: value}}, rules ->
        do_matching_rules(
          value,
          current_path ++ [key],
          rules |> add_rule(current_path ++ [key])
        )

      {key, %{} = value}, rules ->
        do_matching_rules(value, current_path ++ [key], rules)

      # WIP for LISTS
      # {key, values}, rules when is_list(values) ->
      #   do_matching_rules(values, current_path ++ [key], rules)

      {key, value}, rules ->
        rules
    end)
  end

  def do_matching_rules(content, current_path, rules) when is_list(content) do
    content
    |> Enum.with_index()
    |> IO.inspect()
    |> Enum.reduce(rules, fn {value, index}, rules ->
      do_matching_rules(value, current_path ++ [index], rules)
    end)
  end

  def do_matching_rules(_content, _current_path, rules), do: rules

  def add_rule(rules, paths) do
    rules |> Map.put(Enum.join(paths, "."), %{"match" => "type"})
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
        status: status
        # matchingRules: PactElixir.Response.matching_rules(response)
      },
      options
    )
  end
end
