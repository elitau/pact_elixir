defmodule PactElixir.Term do
  defstruct [:generate, :regex]

  def term(generate, regex) do
    %PactElixir.Term{generate: generate, regex: regex}
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
end
