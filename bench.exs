values = Enum.to_list(1..10)

{:ok, redix} = Redix.start_link("redis://localhost:6379")
{:ok, client} = RedRS.open("redis://localhost:6379")
{:ok, redrs} = RedRS.get_connection(client)

Benchee.run(
  %{
    "redix set/get" => fn ->
    for idx <- values do
      key = "redix#{idx}"
      Redix.command!(redix, ["SET", key, to_string(idx)])
      Redix.command!(redix, ["GET", key])
    end
  end,
    "redrs set/get" => fn ->
      for idx <- values do
        key = "redrs#{idx}"
        :ok = RedRS.set(redrs, key, to_string(idx))
        {:ok, _} = RedRS.get(redrs, key)
      end
    end
  },
  time: 10,
  memory_time: 2,
  reduction_time: 2
)

RedRS.close(client)
Redix.stop(redix)
