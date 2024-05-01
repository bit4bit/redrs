defmodule RedRS do
  @moduledoc """
  Documentation for `Redrs`.
  """

  use Rustler, otp_app: :redrs, crate: "redrs"

  def open(_url), do: :erlang.nif_error(:nif_not_loaded)
  def get_connection(_client), do: :erlang.nif_error(:nif_not_loaded)
  def close(_conn), do: :erlang.nif_error(:nif_not_loaded)
  def get(_conn, _key), do: :erlang.nif_error(:nif_not_loaded)
  def set(_conn, _key, _value), do: :erlang.nif_error(:nif_not_loaded)
end
