#include <springql.h>
#include <assert.h>
#include <string.h>
#include <stdio.h>

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
        "  REMOTE_HOST 'localhost',"
        "  REMOTE_PORT '19876'"
        ");");
    assert(ret == Ok);

    ret = spring_command(
        pipeline,
        "CREATE PUMP pu_projection AS"
        "  INSERT INTO sink_trade (ts, amount)"
        "  SELECT STREAM ts, amount FROM source_trade;");
    assert(ret == Ok);

    ret = spring_command(
        pipeline,
        "CREATE SINK STREAM sink_trade ("
        "  ts TIMESTAMP NOT NULL,"
        "  ticker TEXT NOT NULL,"
        "  amount INTEGER NOT NULL"
        ") SERVER IN_MEMORY_QUEUE OPTIONS ("
        "  NAME 'q_sink_trade'"
        ");");
    assert(ret == Ok);

    ret = spring_command(
        pipeline,
        "ALTER PUMP pu_projection START;");
    assert(ret == Ok);
}

void pop_print(const SpringPipeline *pipeline)
{
    const int ts_len = 128;
    const char ts[ts_len];

    for (int i = 0; i < 5; ++i)
    {
        SpringErrno ret;

        SpringRow row;
        ret = spring_pop(pipeline, "q_sink_trade", &row);
        assert(ret == Ok);

        int r = spring_column_text(&row, 0, (char *)ts, ts_len);
        assert(r == strlen(ts));

        int amount;
        ret = spring_column_int(&row, 1, &amount);
        assert(ret == Ok);

        printf("[row#%d] ts=%s amount=%d\n", i, ts, amount);

        spring_row_close(&row);
    }
}

int main()
{
    SpringErrno ret;

    SpringPipeline pipeline;
    ret = spring_open(&pipeline);
    assert(ret == Ok);

    setup_pipeline(&pipeline);

    pop_print(&pipeline);

    // Close the pipeline
    ret = spring_close(&pipeline);
    assert(ret == Ok);

    return 0;
}
