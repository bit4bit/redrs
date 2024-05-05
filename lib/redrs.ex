defmodule RedRS do
  alias RedRSNif, as: NIF

  defdelegate open(conn), to: NIF

  def get_connection(conn) do
    NIF.get_connection(conn, 5000)
  end

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
    case NIF.command(conn, self(), List.wrap(cmd) |> Enum.map(&to_string/1)) do
      :ok ->
        receive do
          {:redrs, :ok, value} ->
            {:ok, value}

          {:redrs, :error, error} ->
            {:error, error}
        end
      {:error, error} ->
        {:error, error}
    end
  end
end
