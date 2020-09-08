defmodule Beamcoin.Mixfile do
  use Mix.Project

  def project do
    [
      app: :beamcoin,
      version: "0.1.0",
      elixir: "~> 1.5",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler] ++ Mix.compilers(),
      deps: deps(),
      rustler_crates: [beamcoin: []]
    ]
  end

  def application do
    [mod: {Beamcoin.Application, []}, extra_applications: [:logger]]
  end

  defp deps do
    [{:rustler, "~> 0.22-rc"}]
  end
end
