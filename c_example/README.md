## How to run

### Running a TCP server to generate trade data in JSON

```bash
python print_trade.py | nc -l 19876
```

### Build and run the example

```bash
cmake . && make && ./run_example
```
