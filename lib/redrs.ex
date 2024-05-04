defmodule RedRS do
  alias RedRSNif, as: NIF

  defdelegate open(conn), to: NIF
  defdelegate get_connection(conn), to: NIF
  defdelegate close(conn), to: NIF

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
    NIF.command(conn, List.wrap(cmd) |> Enum.map(&to_string/1))
  end
end
