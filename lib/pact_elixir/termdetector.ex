defmodule PactElixir.TermDetector do
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
end
