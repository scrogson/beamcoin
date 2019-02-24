defmodule Beamcoin.Native do
  use Rustler, otp_app: :beamcoin

  def start, do: :erlang.nif_error(:nif_not_loaded)
  def mine(_resource), do: :erlang.nif_error(:nif_not_loaded)
  def stop(_resource), do: :erlang.nif_error(:nif_not_loaded)
end
