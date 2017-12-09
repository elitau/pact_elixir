# TODO: This test has a broken usage of ports.
defmodule PactElixir.RustPactMockServerFacadeTest do
  use ExUnit.Case
  alias PactElixir.RustPactMockServerFacade

  @pact """
  {
        "provider": {
          "name": "test_provider"
        },
        "consumer": {
          "name": "test_consumer"
        },
        "interactions": [
          {
            "providerState": "test state",
            "description": "test interaction",
            "request": {
              "method": "GET",
              "path": "/call_me"
            },
            "response": {
              "status": 200,
              "body": "Stop calling me"
            }
          }
        ],
        "metadata": {
          "pact-specification": {
            "version": "2.0.0"
          }
        }
      }
  """
  @port 50823

  describe "create_mock_server" do
    test "creates a mock server and returns its port" do
      assert {:ok, @port} == RustPactMockServerFacade.create_mock_server(@pact, @port)
      assert "Stop calling me" == get_request("/call_me").body
      RustPactMockServerFacade.cleanup_mock_server(@port)
    end

    test "fails if mock server could not start due to broken json" do
      broken_json = "broken{}json"

      assert {:error, :invalid_pact_json} =
               RustPactMockServerFacade.create_mock_server(broken_json, @port)
    end

    # TODO: Find a way to trigger this error
    # test "returns error if mock server could not start because of some general error" do
    #   assert {:error, :mock_server_failed_to_start} = RustPactMockServerFacade.create_mock_server(@pact, 22)

    #   RustPactMockServerFacade.cleanup_mock_server(@port)
    # end
  end

  describe "mock_server_mismatches" do
    test "returns mismatches json when no requests were made" do
      port = 50824
      RustPactMockServerFacade.create_mock_server(@pact, port)

      assert {:ok, mismatches_json_string} = RustPactMockServerFacade.mock_server_mismatches(port)

      assert String.ends_with?(mismatches_json_string, "}]")
      RustPactMockServerFacade.cleanup_mock_server(port)
    end
  end

  describe "matched" do
    test "returns false if none of expected requests were made" do
      port = 50828
      RustPactMockServerFacade.create_mock_server(@pact, port)
      assert {:ok, false} = RustPactMockServerFacade.mock_server_matched(port)
      RustPactMockServerFacade.cleanup_mock_server(port)
    end
  end

  describe "write_pact_file" do
    test "writes pact file" do
      {:ok, dir_path} = Temp.mkdir("RustPactMockServerFacadeTest")
      port = 50825
      RustPactMockServerFacade.create_mock_server(@pact, port)
      assert :ok = RustPactMockServerFacade.write_pact_file(port, dir_path)
      RustPactMockServerFacade.cleanup_mock_server(port)
    end

    test "returns error if there is no mock server for port" do
      {:ok, dir_path} = Temp.mkdir("RustPactMockServerFacadeTest")

      assert {:error, :no_mock_server_running_on_port} =
               RustPactMockServerFacade.write_pact_file(@port - 1000, dir_path)
    end

    test "returns error if io could not complete" do
      port = 50826
      RustPactMockServerFacade.create_mock_server(@pact, port)

      assert {:error, :io_error} =
               RustPactMockServerFacade.write_pact_file(port, "/not/existing/path")

      RustPactMockServerFacade.cleanup_mock_server(port)
    end
  end

  describe "cleanup_mock_server" do
    test "returns true" do
      port = 50827
      RustPactMockServerFacade.create_mock_server(@pact, port)
      assert {:ok, true} == RustPactMockServerFacade.cleanup_mock_server(port)
    end
  end

  def get_request(path) do
    %HTTPoison.Response{} = HTTPoison.get!("http://localhost:#{@port}#{path}")
  end
end
