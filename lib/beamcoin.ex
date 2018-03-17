defmodule Beamcoin do
  use Rustler, otp_app: :beamcoin

  require Logger

  def mine do
    :ok = native_mine()

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

  def native_mine, do: :erlang.nif_error(:nif_not_loaded)
end
