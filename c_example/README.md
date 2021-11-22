## Writing an SpringQL C application (embedded mode)

See [`example.c`](example.c)

## Building

You need to put `springql.h` to an include path and `libspringql_client.{so,dylib}` to a library path first. Then,

```bash
gcc example.c -lspringql_client -o run_example
```

If you haven't downloaded the header and shared library files, then,

```bash
(cd ..; cargo build)  # build shared library
gcc example.c -lspringql_client -o run_example -I.. -L../target/debug
```

You can also use CMake here.

```bash
(cd ..; cargo build)  # build shared library
cmake . && make
```

## Running the sample application

First, you need to run a source server to generate trade data.
You may need to install `nc` (netcat) to your system.

```bash
python print_trade.py | nc -l 19876  # running a source server on TCP port 19876
```

Finally the example app runs.

```bash
./run_example

[row#0] ts=2021-11-23 05:38:32.589299000 amount=200
[row#1] ts=2021-11-23 05:38:32.590267000 amount=100
[row#2] ts=2021-11-23 05:38:32.590284000 amount=900
[row#3] ts=2021-11-23 05:38:32.590295000 amount=500
[row#4] ts=2021-11-23 05:38:32.590305000 amount=100
```
