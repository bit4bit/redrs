defmodule RedRSTest do
  use ExUnit.Case

  @moduletag :external

  test "open and close" do
    {:ok, client} = RedRS.open("redis://127.0.0.1/")
    {:ok, _} = RedRS.get_connection(client)
    assert RedRS.close(client) == :ok
  end

  test "set and get" do
    {:ok, client} = RedRS.open("redis://127.0.0.1/")
    {:ok, conn} = RedRS.get_connection(client)

    assert :ok == RedRS.set(conn, "name", "mero")
    assert {:ok, "mero"} == RedRS.get(conn, "name")

    RedRS.close(client)
  end

  # TODO how can we guarantee closes?
  @tag skip: true
  describe "get_connection/1" do
    test "when invalid returns error" do
      {:ok, client} = RedRS.open("redis://127.0.0.1/")
      {:ok, _conn} = RedRS.get_connection(client)
      RedRS.close(client)

      assert {:error, _} = RedRS.get_connection(client)
    end
  end

  describe "open/1" do
    test "when invalid url returns error" do
      assert {:error, "Redis URL did not parse- InvalidClientConfig"} = RedRS.open("redisa://127.0.0.5353/")
    end
  end

  describe "get/2" do
    setup do
      {:ok, client} = RedRS.open("redis://127.0.0.1/")
      {:ok, conn} = RedRS.get_connection(client)
      [conn: conn]
    end

    test "not found key", %{conn: conn} do
      assert {:ok, nil} == RedRS.get(conn, "invalid")
    end
  end
end
