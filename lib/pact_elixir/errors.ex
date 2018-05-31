defmodule PactElixir.MismatchesError do
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
  @moduledoc false
  def format_mismatches(mismatches) do
    mismatches
    |> Enum.map(fn %{
                     "actual" => actual,
                     "expected" => expected,
                     "mismatch" => mismatch,
                     "parameter" => _parameter,
                     "type" => _type
                   } ->
      ~s|#{mismatch}. Expected: "#{expected}", got "#{actual}".|
    end)
    |> Enum.join("\n")
  end
end

defmodule PactElixir.VerificationError do
  defexception [:message]
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

  def convert_to_error(mismatches) do
    messages = Enum.map(mismatches, &mismatch_to_message/1)

    if messages != [] do
      raise PactElixir.VerificationError,
            "error while verifying expectations\n\n" <> Enum.join(messages, "\n")
    end

    :ok
  end

  defp mismatch_to_message(%{"request" => request, "type" => type} = _mismatch) do
    "#{type_to_name(type)}:\n#{map_to_string(request)}"
  end

  defp mismatch_to_message(
         %{"mismatches" => mismatches, "type" => type, "method" => method, "path" => path} =
           _mismatch
       ) do
    "#{type_to_name(type)}: Mismatches for #{method} '#{path}':\n" <>
      PactElixir.MismatchesError.format_mismatches(mismatches)
  end

  def type_to_name(type) do
    String.split(type, "-")
    |> Enum.map(&String.capitalize/1)
    |> Enum.join(" ")
  end

  def map_to_string(map) do
    # Poison.encode!(map, pretty: true)
    Poison.encode!(map)
  end
end

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
