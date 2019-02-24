defmodule Beamcoin do

  require Logger

  def demo do
    Task.start(fn -> mine() end)
  end

  def mine do
    {:ok, resource} = Beamcoin.Server.get_resource()
    :ok = Beamcoin.Native.mine(resource)

    start = System.system_time(:seconds)
    receive do
      {:ok, number, hash} ->
        done = System.system_time(:seconds)
        total = done - start
        Logger.info("Solution found in #{total} seconds")
        Logger.info("The number is: #{number}")
        Logger.info("The hash is: #{hash}")
      {:error, reason} ->
        Logger.error(inspect reason)
    end
  end

  def stop do
    GenServer.stop(Beamcoin.Server)
  end
end
