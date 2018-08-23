defmodule PactElixir.MockServerSupervisorTest do
  use ExUnit.Case
  alias PactElixir.{ServiceProvider, MockServerSupervisor}

  test "MockServerSupervisor is running after app started" do
    mock_supervisor_pid = Process.whereis(MockServerSupervisor)
    assert Process.info(mock_supervisor_pid)
  end

  test "start a supervised PactMockServer for a service provider" do
    provider = ServiceProvider.new()
    assert {:ok, pid} = MockServerSupervisor.start_mock_server(provider)

    assert is_pid(pid)

    MockServerSupervisor.terminate_child(pid)
  end

  test "terminate supervised PactMockServer" do
    provider = ServiceProvider.new()
    {:ok, pid} = MockServerSupervisor.start_mock_server(provider)

    assert %{active: 1} = DynamicSupervisor.count_children(MockServerSupervisor)

    MockServerSupervisor.terminate_child(pid)

    assert %{active: 0} = DynamicSupervisor.count_children(MockServerSupervisor)
  end
end
