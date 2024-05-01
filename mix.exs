defmodule Redrs.MixProject do
  use Mix.Project

  def project do
    [
      app: :redrs,
      version: "0.1.0",
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.32.1"},
      {:benchee, "~> 1.0", only: :dev},
      {:redix, "~> 1.5", only: :dev}
    ]
  end
end
