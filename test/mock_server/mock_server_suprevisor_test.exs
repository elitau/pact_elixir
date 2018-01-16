defmodule PactElixir.MockServerSupervisorTest do
  use ExUnit.Case

  test "MockServerSupervisor is running after app started" do
    mock_supervisor_pid = Process.whereis(PactElixir.MockServerSupervisor)
    assert Process.info(mock_supervisor_pid)
  end

  test "start a supervised PactMockServer for a service provider" do
    provider = PactElixir.ServiceProvider.new()
    assert {:ok, pid} = PactElixir.MockServerSupervisor.start_mock_server(provider)

    assert is_pid(pid)
  end
end
