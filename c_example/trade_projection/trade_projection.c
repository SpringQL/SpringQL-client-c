// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

// Usage:
//
// $ python print_trade.py | nc -l 19876  # running a source server on TCP port 19876
// $ ./a.out

#include <springql.h>
#include <assert.h>
#include <string.h>
#include <stdio.h>

void abort_with_report()
{
    SpringErrno errno_;
    char errmsg[1024];
    spring_last_err(&errno_, errmsg, 1024);
    fprintf(stderr, "Error occurred (%d): %s", errno_, errmsg);
    abort();
}

void assert_ok(SpringErrno ret)
{
    if (ret != Ok)
    {
        abort_with_report();
    }
}

void assert_not_null(void *p)
{
    if (p == NULL)
    {
        abort_with_report();
    }
}

void setup_pipeline(const SpringPipeline *pipeline)
{
    SpringErrno ret;

    ret = spring_command(
        pipeline,
        "CREATE SOURCE STREAM source_trade ("
        "  ts TIMESTAMP NOT NULL ROWTIME,"
        "  ticker TEXT NOT NULL,"
        "  amount INTEGER NOT NULL"
        ");");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK STREAM sink_trade ("
        "  ts TIMESTAMP NOT NULL,"
        "  amount INTEGER NOT NULL"
        ");");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE PUMP pu_projection AS"
        "  INSERT INTO sink_trade (ts, amount)"
        "  SELECT STREAM source_trade.ts, source_trade.amount FROM source_trade;");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK WRITER in_mem_queue_sink_trade FOR sink_trade"
        "  TYPE IN_MEMORY_QUEUE OPTIONS ("
        "    NAME 'q_sink_trade'"
        "  );");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SOURCE READER tcp_source_trade FOR source_trade"
        "  TYPE NET_CLIENT OPTIONS ("
        "    PROTOCOL 'TCP',"
        "    REMOTE_HOST '127.0.0.1',"
        "    REMOTE_PORT '19876'"
        "  );");
    assert_ok(ret);
}

void pop_print(const SpringPipeline *pipeline)
{
#define TS_LEN 128
    const char ts[TS_LEN];

    for (int i = 0; i < 5; ++i)
    {
        SpringErrno ret;

        SpringSinkRow *row = spring_pop(pipeline, "q_sink_trade");
        assert_not_null(row);

        int r = spring_column_text(row, 0, (char *)ts, TS_LEN);
        assert((size_t)r == strlen(ts));

        int amount;
        ret = spring_column_int(row, 1, &amount);
        assert_ok(ret);

        printf("[row#%d] ts=%s amount=%d\n", i, ts, amount);

        spring_sink_row_close(row);
    }
}

int main()
{
    SpringErrno ret;

    SpringConfig *config = spring_config_toml(
        "[memory]\n"
        "upper_limit_bytes = 1_000_000\n");
    assert_not_null(config);

    SpringPipeline *pipeline = spring_open(config);
    assert_not_null(pipeline);

    setup_pipeline(pipeline);

    pop_print(pipeline);

    ret = spring_close(pipeline);
    assert_ok(ret);

    ret = spring_config_close(config);
    assert_ok(ret);

    return 0;
}
