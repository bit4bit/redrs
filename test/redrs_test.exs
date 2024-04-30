defmodule RedRSTest do
  use ExUnit.Case

  @moduletag :external

  test "connect and close" do
    conn = RedRS.open("redis://127.0.0.1/") 
    assert RedRS.close(conn)
  end
end
