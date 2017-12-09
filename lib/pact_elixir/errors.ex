defmodule PactElixir.RequestError do
  @moduledoc """
  Exception for request errors.
  """
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
  @moduledoc """
  Currently this is the whole error handling stuff.
  """

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
    "RequestError: #{exception.message}, method: #{exception.method}, path: #{exception.path}, request: #{
      inspect(exception.request)
    }"
  end

  # %{
  #   "method" => "GET",
  #   "path" => "/organizations/org23/locations/betrieb41",
  #   "request" => %{
  #     "body" => "Empty",
  #     "headers" => %{
  #       "connection" => "keep-alive",
  #       "content-length" => "0",
  #       "host" => "localhost:61627",
  #       "te" => ""
  #     },
  #     "matching_rules" => %{"rules" => %{}},
  #     "method" => "GET",
  #     "path" => "/organizations/org23/locations/betrieb41",
  #     "query" => nil
  #   },
  #   "type" => "request-not-found"
  # }

  # MethodMismatch {
  #     /// Expected request method
  #     expected: String,
  #     /// Actual request method
  #     actual: String
  # },
  # /// Request Path mismatch
  # PathMismatch {
  #     /// expected request path
  #     expected: String,
  #     /// actual request path
  #     actual: String,
  #     /// description of the mismatch
  #     mismatch: String
  # },
  # /// Response status mismatch
  # StatusMismatch {
  #     /// expected response status
  #     expected: u16,
  #     /// actual response status
  #     actual: u16
  # },
  # /// Request query mismatch
  # QueryMismatch {
  #     /// query parameter name
  #     parameter: String,
  #     /// expected value
  #     expected: String,
  #     /// actual value
  #     actual: String,
  #     /// description of the mismatch
  #     mismatch: String
  # },
  # /// Header mismatch
  # HeaderMismatch {
  #     /// header key
  #     key: String,
  #     /// expected value
  #     expected: String,
  #     /// actual value
  #     actual: String,
  #     /// description of the mismatch
  #     mismatch: String
  # },
  # /// Mismatch in the content type of the body
  # BodyTypeMismatch {
  #     /// expected content type of the body
  #     expected: String,
  #     /// actual content type of the body
  #     actual: String
  # },
  # /// Body element mismatch
  # BodyMismatch {
  #     /// path expression to where the mismatch occured
  #     path: String,
  #     /// expected value
  #     expected: Option<Vec<u8>>,
  #     /// actual value
  #     actual: Option<Vec<u8>>,
  #     /// description of the mismatch
  #     mismatch: String
  # }

  # RequestMatch
  # RequestMismatch
  # RequestNotFound
  # MissingRequest

  # pub enum DifferenceType {
  #   /// Methods differ
  #   Method,
  #   /// Paths differ
  #   Path,
  #   /// Headers differ
  #   Headers,
  #   /// Query parameters differ
  #   QueryParameters,
  #   /// Bodies differ
  #   Body,
  #   /// Matching Rules differ
  #   MatchingRules,
  #   /// Response status differ
  #   Status
  # }
  # defp to_error(%{
  #        "method" => "GET",
  #        "path" => "/organizations/org23/locations/betrieb41",
  #        "request" => %{
  #          "body" => "Empty",
  #          "headers" => %{
  #            "connection" => "keep-alive",
  #            "content-length" => "0",
  #            "host" => "localhost:61627",
  #            "te" => ""
  #          },
  #          "matching_rules" => %{"rules" => %{}},
  #          "method" => "GET",
  #          "path" => "/organizations/org23/locations/betrieb41",
  #          "query" => nil
  #        },
  #        "type" => "request-not-found"
  #      }) do
  # end

  defp mismatch_to_error(mismatch) do
    case mismatch do
      %{"type" => "missing-request"} ->
        %PactElixir.RequestError{
          method: mismatch["method"],
          path: mismatch["path"],
          request: mismatch["request"],
          message: "Missing request"
        }

      %{"type" => "request-not-found"} ->
        %PactElixir.RequestError{
          method: mismatch["method"],
          path: mismatch["path"],
          request: mismatch["request"],
          message: "Request not found"
        }

      _ ->
        mismatch
    end
  end
end
