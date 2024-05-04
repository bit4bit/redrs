defmodule RedRS do
  alias RedRSNif, as: NIF

  defstruct [:conn, :reply_pid]

  defdelegate open(conn), to: NIF

  def get_connection(conn) do
    reply_pid = self()
    case NIF.get_connection(conn, reply_pid) do
      {:ok, rconn} ->
        {:ok, %__MODULE__{conn: rconn, reply_pid: reply_pid}}
      {:error, error} ->
        {:error, error}
    end
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

  def command(%__MODULE__{conn: conn, reply_pid: reply_pid}, cmd) do
    # only string is supported
    NIF.command(conn, List.wrap(cmd) |> Enum.map(&to_string/1))

    receive do
      {:redrs, :ok, value} ->
        IO.inspect(value)
        {:ok, value}

      {:redrs, :error, error} ->
        {:error, error}
    after
      1_000 ->
        raise "timeout"
    end
  end
end
