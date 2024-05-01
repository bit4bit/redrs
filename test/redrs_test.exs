defmodule RedRSTest do
  use ExUnit.Case

  @moduletag :external

  test "open and close" do
    {:ok, conn} = RedRS.open("redis://127.0.0.1/") 
    assert RedRS.close(conn) == :ok
  end

  test "set and get" do
    {:ok, conn} = RedRS.open("redis://127.0.0.1/")

    assert :ok == RedRS.set(conn, "name", "mero")
    assert {:ok, "mero"} == RedRS.get(conn, "name")

    RedRS.close(conn)
  end

  describe "open/1" do
    test "when invalid url returns error" do
      assert {:error, "Redis URL did not parse- InvalidClientConfig"} = RedRS.open("redisa://127.0.0.5353/")
    end
  end

  describe "get/2" do
    setup do
      {:ok, conn} = RedRS.open("redis://127.0.0.1/")
      [conn: conn]
    end

    test "not found key", %{conn: conn} do
      assert {:ok, nil} == RedRS.get(conn, "invalid")
    end
  end
end
