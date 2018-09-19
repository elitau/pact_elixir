defmodule TermDetector do

  def recursively_update_terms(%Term{} = to_update) do
    to_update_new = Term.get_my_map(to_update)

    _recursively_update_terms(to_update_new)
  end

  def recursively_update_terms(%{} = to_update),
    do: _recursively_update_terms(to_update)

  def _recursively_update_terms(%{} = to_update) do
    to_update
    |> Map.to_list()
    |> Enum.map(fn
                {k, %Term{} = v} -> {k, recursively_update_terms(v)}
                {k, %{} = v} -> {k, _recursively_update_terms(v)}
                {k, v} -> {k, v}
              end)
    |> Enum.into(%{})
  end

  defp random_thing(),
    do: :crypto.strong_rand_bytes(10)
    |> Base.url_encode64(padding: false)
end

# Run these to understand how the module works
#TermDetector.recursively_update_terms(%{a: "hey", b: "ho", x: %Term{}, c: "zzz"})
#TermDetector.recursively_update_terms(%{a: "hey", b: "ho", x: %Term{regex: "someawesomeregex", generate: %Term{generate: %Term{generate: "1", regex: "somegreatregex"}, regex: "somecoolregex"}}, c: "zzz"})
#TermDetector.recursively_update_terms(%Term{})
#TermDetector._recursively_update_terms(%Term{})