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

  def matching_rules(body), do: do_matching_rules({:body, body}, [:"$"], %{})

  def do_matching_rules({path, %PactElixir.TypeMatcher{value: value}}, previous_paths, rules) do
    do_matching_rules({path, value}, previous_paths, rules |> add_rule(path, previous_paths))
  end

  def do_matching_rules({path, content}, previous_paths, rules) when is_map(content) do
    content
    |> Enum.reduce(rules, fn {key, value}, rules ->
      do_matching_rules({key, value}, previous_paths ++ [path], rules)
    end)
  end

  def do_matching_rules({path, values}, previous_paths, rules) when is_list(values) do
    values
    |> Enum.with_index()
    |> Enum.reduce(rules, fn {value, index}, rules ->
      do_matching_rules({key_for_list_element(path, index), value}, previous_paths, rules)
    end)
  end

  def do_matching_rules({_path, value}, _previous_paths, _rules) when is_tuple(value),
    do: raise(ArgumentError, "Tuples are not supported. Given #{value |> inspect()}")

  def do_matching_rules(_content, _previous_paths, rules), do: rules

  def add_rule(rules, key, previous_paths) do
    rules |> Map.put(Enum.join(previous_paths ++ [key], "."), %{"match" => "type"})
  end

  def key_for_list_element(path, index) do
    "#{path}[#{index}]"
  end
end

defimpl Poison.Encoder, for: PactElixir.Response do
  def encode(
        %PactElixir.Response{
          body: body,
          headers: headers,
          status: status,
          matching_rules: matching_rules
        },
        options
      ) do
    Poison.Encoder.Map.encode(
      %{
        body: body,
        headers: headers,
        status: status,
        matchingRules: matching_rules
      },
      options
    )
  end
end
