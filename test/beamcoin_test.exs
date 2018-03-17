defmodule BeamcoinTest do
  use ExUnit.Case
  doctest Beamcoin

  test "greets the world" do
    assert Beamcoin.hello() == :world
  end
end
