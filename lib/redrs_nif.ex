defmodule RedRSNif do
  @moduledoc """
  Documentation for `Redrs`.
  """

  use Rustler, otp_app: :redrs, crate: "redrs"

  def open(_url), do: :erlang.nif_error(:nif_not_loaded)
  def get_connection(_client, _timeout), do: :erlang.nif_error(:nif_not_loaded)
  def close(_conn), do: :erlang.nif_error(:nif_not_loaded)
  def command(_conn, _ref, _reply_pid, _cmd), do: :erlang.nif_error(:nif_not_loaded)
end
