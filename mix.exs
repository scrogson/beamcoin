defmodule Beamcoin.Mixfile do
  use Mix.Project

  def project do
    [app: :beamcoin,
     version: "0.1.0",
     elixir: "~> 1.5",
     start_permanent: Mix.env == :prod,
     compilers: [:rustler] ++ Mix.compilers(),
     deps: deps(),
     rustler_crates: rustler_crates()]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [{:rustler, github: "hansihe/rustler", sparse: "rustler_mix"}]
  end

  defp rustler_crates do
    [beamcoin: [
      path: "native/beamcoin",
      mode: mode(Mix.env)
    ]]
  end

  defp mode(:prod), do: :release
  defp mode(_), do: :debug
end
