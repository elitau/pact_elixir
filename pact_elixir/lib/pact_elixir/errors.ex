defmodule PactElixir.RequestError do
  @no_value :pact_elixir_no_meaningful_value
  defexception path: @no_value,
               method: @no_value,
               message: @no_value,
               request: @no_value

  def message(exception) do
    "\n\n" <> PactElixir.Errors.format_difference_error(exception)
  end
end

defmodule PactElixir.Errors do
  # %{
  #   "method" => "GET",
  #   "path" => "/foo",
  #   "request" => %{
  #     "body" => "Missing",
  #     "headers" => nil,
  #     "matching_rules" => nil,
  #     "method" => "GET",
  #     "path" => "/foo",
  #     "query" => nil
  #   },
  #   "type" => "missing-request"
  # }
  def convert_to_errors(mismatches) do
    Enum.map(mismatches, &mismatch_to_error/1)
  end

  def format_difference_error(%PactElixir.RequestError{} = exception) do
    "RequestError: method: #{exception.method}, path: #{exception.path}"
  end

  defp mismatch_to_error(mismatch) do
    case mismatch do
      %{"type" => "missing-request"}
        -> %PactElixir.RequestError{
             method: mismatch["method"],
             path: mismatch["path"],
             request: mismatch["request"],
             message: "Missing request"
           }
      _ -> mismatch
    end
  end
end