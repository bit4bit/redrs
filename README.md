# Redrs

Lab - Experimental client using `redis-rust`.

`mix run benchee.exs`

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `redrs` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:redrs, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/redrs>.

## Result

There is no tangible benefit to replacing Redix.

```Comparison: 
CPU Information: Intel(R) Core(TM) i5-3320M CPU @ 2.60GHz
Number of Available Cores: 4
Available memory: 15.54 GB
Elixir 1.16.1
Erlang 26.2.2
JIT enabled: true

Benchmark suite executing with the following configuration:
warmup: 2 s
time: 10 s
memory time: 2 s
reduction time: 2 s
parallel: 1
inputs: none specified
Estimated total run time: 32 s

Benchmarking redix set/get ...
Benchmarking redrs set/get ...
Calculating statistics...
Formatting results...

Name                    ips        average  deviation         median         99th %
redrs set/get        413.40        2.42 ms    ±18.88%        2.29 ms        4.16 ms
redix set/get        356.01        2.81 ms    ±30.57%        2.52 ms        5.95 ms

Comparison: 
redrs set/get        413.40
redix set/get        356.01 - 1.16x slower +0.39 ms

Memory usage statistics:

Name             Memory usage
redrs set/get         3.41 KB
redix set/get        27.11 KB - 7.96x memory usage +23.70 KB

**All measurements for memory usage were the same**

Reduction count statistics:

Name                  average  deviation         median         99th %
redrs set/get          0.79 K     ±0.16%         0.79 K         0.79 K
redix set/get          5.08 K     ±0.00%         5.08 K         5.08 K

Comparison: 
redrs set/get          0.79 K
redix set/get          5.08 K - 6.41x reduction count +4.29 K
```
