defmodule PactElixir.ErrorsTest do
  use ExUnit.Case
  alias PactElixir.VerificationError
  import PactElixir.Errors

  test "outputs request errors" do
    mismatches = [missing_request_error()]

    assert_raise VerificationError, ~r/Missing Request.*\/foo.*/s, fn ->
      convert_to_error(mismatches)
    end
  end

  test "outputs request mismatch errors" do
    mismatches = [request_mismatch_error()]

    assert_raise VerificationError, ~r/Request Mismatch.*\/path.*/s, fn ->
      convert_to_error(mismatches)
    end
  end

  test "outputs request not found errors" do
    mismatches = [request_not_found()]

    assert_raise VerificationError, ~r/Request Not Found.*\/organizations.*/s, fn ->
      convert_to_error(mismatches)
    end
  end

  # request received but has mismatches
  defp request_mismatch_error do
    %{
      "method" => "GET",
      "mismatches" => [
        %{
          "actual" => "",
          "expected" => "",
          "mismatch" => "Unexpected query parameter 'filter' received",
          "parameter" => "filter",
          "type" => "QueryMismatch"
        }
      ],
      "path" => "/path",
      "type" => "request-mismatch"
    }
  end

  # interaction ? expected, but never occurred
  defp missing_request_error do
    %{
      "method" => "GET",
      "path" => "/foo",
      "request" => %{
        "body" => "Missing",
        "headers" => nil,
        "matching_rules" => nil,
        "method" => "GET",
        "path" => "/foo",
        "query" => nil
      },
      "type" => "missing-request"
    }
  end

  # received unexpected request
  defp request_not_found do
    %{
      "method" => "GET",
      "path" => "/organizations/org23/locations/betrieb41",
      "request" => %{
        "body" => "Empty",
        "headers" => %{
          "connection" => "keep-alive",
          "content-length" => "0",
          "host" => "localhost:61627",
          "te" => ""
        },
        "matching_rules" => %{"rules" => %{}},
        "method" => "GET",
        "path" => "/organizations/org23/locations/betrieb41",
        "query" => nil
      },
      "type" => "request-not-found"
    }
  end
end
