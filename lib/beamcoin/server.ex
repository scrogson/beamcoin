defmodule Beamcoin.Server do
  use GenServer

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def get_resource do
    case :ets.lookup(Beamcoin, Beamcoin) do
      [{_, resource}] -> {:ok, resource}
      [] -> {:error, nil}
    end
  end

  def init(_) do
    _ = :ets.new(Beamcoin, [:set, :public, :named_table])
    {:ok, resource} = Beamcoin.Native.start()
    :ets.insert(Beamcoin, {Beamcoin, resource})

    {:ok, nil}
  end

  def terminate(_reason, _) do
    with {:ok, resource} <- get_resource() do
      :ets.delete(Beamcoin, Beamcoin)
      Beamcoin.Native.stop(resource)
    end

    :ok
  end
end
