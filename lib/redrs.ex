defmodule RedRS do
  @moduledoc """
  Documentation for `Redrs`.
  """

  use Rustler, otp_app: :redrs, crate: "redrs"

  def open(_url), do: :erlang.nif_error(:nif_not_loaded)
  def get_connection(_client), do: :erlang.nif_error(:nif_not_loaded)
  def close(_conn), do: :erlang.nif_error(:nif_not_loaded)

  def get(conn, key) do
    command(conn, ["GET", key])
  end

  def set(conn, key, value) do
    case command(conn, ["SET", key, value]) do
      {:ok, _} -> :ok
      {:error, err} -> {:error, err}
    end
  end

  def command(conn, cmd) do
    # only string is supported
    do_command(conn, List.wrap(cmd) |> Enum.map(&to_string/1))
  end

  def do_command(_conn, _cmd), do: :erlang.nif_error(:nif_not_loaded)
end
