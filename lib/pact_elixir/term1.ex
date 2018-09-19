defmodule Term1 do
  defstruct [:generate, :regex]



def get_my_map(%Term1{} = term) do
     %{
      json_class: "Pact::Term",
      data: %{
        generate: term.generate,
        matcher: %{json_class: "Regexp", o: 0, s: term.regex}
      }
    }
  end
end