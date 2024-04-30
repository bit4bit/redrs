defmodule RedRS do
  @moduledoc """
  Documentation for `Redrs`.
  """

  use Rustler, otp_app: :redrs, crate: "redrs"

  def open(_url), do: :erlang.nif_error(:nif_not_loaded)
  def close(_conn), do: :erlang.nif_error(:nif_not_loaded)
end
