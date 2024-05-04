defmodule RedRSTest do
  use ExUnit.Case

  @moduletag :external

  describe "commands" do
    setup do
      {:ok, client} = open("redis://127.0.0.1/")
      {:ok, conn} = RedRS.get_connection(client)
      [conn: conn]
    end

    test "open and close" do
      {:ok, client} = open("redis://127.0.0.1/")
      {:ok, _} = RedRS.get_connection(client)
      assert RedRS.close(client) == :ok
    end

    test "open multiple connections" do
      {:ok, client} = open("redis://127.0.0.1/")

      {:ok, conn1} = RedRS.get_connection(client)
      {:ok, conn2} = RedRS.get_connection(client)
      {:ok, conn3} = RedRS.get_connection(client)

      for conn <- [conn1, conn2, conn3] do
        :ok = RedRS.set(conn, "demo", "demo")
        assert {:ok, "demo"} = RedRS.get(conn, "demo")

        RedRS.close(client)
      end
    end

    test "set and get", %{conn: conn} do
      assert :ok = RedRS.set(conn, "name", "mero")
      assert {:ok, "mero"} == RedRS.get(conn, "name")
    end

    test "command", %{conn: conn} do
      {:ok, _} = RedRS.command(conn, ["SET", "nama", "mera"])
      assert {:ok, "mera"} = RedRS.command(conn, ["GET", "nama"])
    end
  end

  # TODO how can we guarantee closes?
  @tag skip: true
  describe "get_connection/1" do
    test "when invalid returns error" do
      {:ok, client} = open("redis://127.0.0.1/")
      {:ok, _conn} = RedRS.get_connection(client)
      RedRS.close(client)

      assert {:error, _} = RedRS.get_connection(client)
    end
  end

  describe "open/1" do
    test "when invalid url returns error" do
      assert {:error, "Redis URL did not parse- InvalidClientConfig"} =
               open("redisa://127.0.0.5353/")
    end
  end

  describe "get/2" do
    setup do
      {:ok, client} = open("redis://127.0.0.1/")
      {:ok, conn} = RedRS.get_connection(client)
      [conn: conn]
    end

    test "not found key", %{conn: conn} do
      assert {:ok, nil} == RedRS.get(conn, "invalid")
    end
  end

  defp open(url) do
    RedRS.open(url)
  end
end
