defmodule PactElixir.TermDetectorTest do
  alias PactElixir.Term, as: Term
  use ExUnit.Case

  test "detect and convert empty Term in map to desired format" do
    output =
      PactElixir.TermDetector.recursively_update_terms(%{
        field_1: "hey",
        field_2: "ho",
        field_3: %Term{}
      })

    assert output == %{
             field_1: "hey",
             field_2: "ho",
             field_3: %{
               data: %{generate: nil, matcher: %{json_class: "Regexp", o: 0, s: nil}},
               json_class: "Pact::Term"
             }
           }
  end

  test "detect and convert Term in nested map to desired format" do
    output =
      PactElixir.TermDetector.recursively_update_terms(%{
        field_1: "hey",
        field_2: "ho",
        field_3: %Term{
          regex: "someawesomeregex",
          generate: %Term{
            generate: %Term{generate: "1", regex: "somegreatregex"},
            regex: "somecoolregex"
          }
        },
        field_4: "zzz"
      })

    assert output == %{
             field_1: "hey",
             field_2: "ho",
             field_3: %{
               json_class: "Pact::Term",
               data: %{
                 matcher: %{json_class: "Regexp", o: 0, s: "someawesomeregex"},
                 generate: %{
                   json_class: "Pact::Term",
                   data: %{
                     matcher: %{json_class: "Regexp", o: 0, s: "somecoolregex"},
                     generate: %{
                       json_class: "Pact::Term",
                       data: %{
                         matcher: %{json_class: "Regexp", o: 0, s: "somegreatregex"},
                         generate: "1"
                       }
                     }
                   }
                 }
               }
             },
             field_4: "zzz"
           }
  end

  test "detect and convert empty Term to desired format" do
    output = PactElixir.TermDetector.recursively_update_terms(%Term{})

    assert output == %{
             data: %{generate: nil, matcher: %{json_class: "Regexp", o: 0, s: nil}},
             json_class: "Pact::Term"
           }
  end

  test "function for handling non-Term values" do
    output = PactElixir.TermDetector._recursively_update_terms(%Term{})
    assert output == %Term{generate: nil, regex: nil}
  end
end
