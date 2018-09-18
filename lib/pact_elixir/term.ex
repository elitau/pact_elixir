defmodule PactElixir.Term do
  @behaviour Access
  defstruct [generate: '', regex: '']

  def test(%PactElixir.Term{generate: this_is_term}) do
    IO.inspect(this_is_term)
  end

  def new(attributes \\ %{}) do
    value_or_default = &value_from_map(attributes, &1, &2)

    %PactElixir.Term{
       generate: value_or_default.(:generate, ""),
       regex: value_or_default.(:regex, "")
    }
  end

  defp value_from_map(attributes, name, default) do
    attributes[name] || attributes[:"#{name}"] || default
  end
  
  def get_my_map(%PactElixir.Term{} = term) do
     %{
      json_class: "Pact::Term",
      data: %{
        generate: term.generate,
        matcher: %{json_class: "Regexp", o: 0, s: term.regex}
      }
    }
  end

  def fetch(term, key) do
    term
    |> Map.from_struct()
    |> Map.fetch(key)
  end
end
