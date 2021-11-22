#include <springql.h>
#include <assert.h>
#include <string.h>
#include <stdio.h>

void abort_with_report()
{
    SpringErrno errno;
    char errmsg[1024];
    spring_last_err(&errno, errmsg, 1024);
    fprintf(stderr, "Error occurred (%d): %s", errno, errmsg);
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
        ") SERVER NET_SERVER OPTIONS ("
        "  PROTOCOL 'TCP',"
        "  REMOTE_HOST '127.0.0.1',"
        "  REMOTE_PORT '19876'"
        ");");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE PUMP pu_projection AS"
        "  INSERT INTO sink_trade (ts, amount)"
        "  SELECT STREAM ts, amount FROM source_trade;");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK STREAM sink_trade ("
        "  ts TIMESTAMP NOT NULL,"
        "  ticker TEXT NOT NULL,"
        "  amount INTEGER NOT NULL"
        ") SERVER IN_MEMORY_QUEUE OPTIONS ("
        "  NAME 'q_sink_trade'"
        ");");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "ALTER PUMP pu_projection START;");
    assert_ok(ret);
}

void pop_print(const SpringPipeline *pipeline)
{
    const int ts_len = 128;
    const char ts[ts_len];

    for (int i = 0; i < 5; ++i)
    {
        SpringErrno ret;

        SpringRow *row = spring_pop(pipeline, "q_sink_trade");
        assert_not_null(row);

        int r = spring_column_text(row, 0, (char *)ts, ts_len);
        assert(r == strlen(ts));

        int amount;
        ret = spring_column_int(row, 1, &amount);
        assert_ok(ret);

        printf("[row#%d] ts=%s amount=%d\n", i, ts, amount);

        spring_row_close(row);
    }
}

int main()
{
    SpringErrno ret;

    SpringPipeline *pipeline = spring_open();
    assert_not_null(pipeline);

    setup_pipeline(pipeline);

    pop_print(pipeline);

    // Close the pipeline
    ret = spring_close(pipeline);
    assert_ok(ret);

    return 0;
}
