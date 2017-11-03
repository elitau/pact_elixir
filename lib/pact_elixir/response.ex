defmodule PactElixir.Response do
  @derive [Poison.Encoder]
  defstruct [:body, :headers, :status]

  def new(attributes \\ %{}) do
    value_or_default = &value_from_map(attributes, &1, &2)

    %PactElixir.Response{
      body: value_or_default.(:body, ""),
      headers: value_or_default.(:headers, %{}),
      status: value_or_default.(:status, 200)
    }
  end

  defp value_from_map(attributes, name, default) do
    attributes[name] || attributes[:"#{name}"] || default
  end
end
