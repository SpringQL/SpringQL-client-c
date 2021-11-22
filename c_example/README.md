## How to run

### Running a TCP server to generate trade data in JSON

```bash
bash -c "trap exit INT; while :; do python print_trade.py | nc -l 19876; done"
```

### Build and run the example

```bash
cmake . && make && ./run_example
```
