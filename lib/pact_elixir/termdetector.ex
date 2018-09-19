defmodule TermDetector do

  def recursively_update_terms(%Term1{} = to_update) do
    to_update_new = Term1.get_my_map(to_update)

    _recursively_update_terms(to_update_new)
  end

  def recursively_update_terms(%{} = to_update),
    do: _recursively_update_terms(to_update)

  def _recursively_update_terms(%{} = to_update) do
    to_update
    |> Map.to_list() # Structs aren't enumerable
    |> Enum.map(fn
                {k, %Term1{} = v} -> {k, recursively_update_terms(v)}
                {k, %{} = v} -> {k, _recursively_update_terms(v)}
                {k, v} -> {k, v}
              end)
    |> Enum.into(%{})
  end

  defp random_thing(),
    do: :crypto.strong_rand_bytes(10)
    |> Base.url_encode64(padding: false)
end

TermDetector.recursively_update_terms(%{a: "hey", b: "ho", x: %Term1{}, c: "zzz"})
TermDetector.recursively_update_terms(%{a: "hey", b: "ho", x: %Term1{regex: "123", generate: %Term1{generate: %Term1{generate: "1", regex: "2"}, regex: 2}}, c: "zzz"})
TermDetector.recursively_update_terms(%Term1{})
TermDetector._recursively_update_terms(%Term1{})