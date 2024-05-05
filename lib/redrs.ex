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
    ref = make_ref()

    case NIF.command(conn, ref, self(), List.wrap(cmd) |> Enum.map(&to_string/1)) do
      :ok ->
        receive do
          {:redrs, :ok, ^ref, value} ->
            {:ok, value}

          {:redrs, :error, ^ref, error} ->
            {:error, error}
        after
          5_000 ->
            raise "NIF timeout"
        end

      {:error, error} ->
        {:error, error}
    end
  end
end
