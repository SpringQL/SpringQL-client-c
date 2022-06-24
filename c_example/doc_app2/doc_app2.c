// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

// Usage:
//
// $ ./a.out  # waiting for connection...
// $ echo '{"ts": "2022-01-01 13:00:00.000000000", "symbol": "ORCL", "amount": 10}' |nc localhost 54300
// $ echo '{"ts": "2022-01-01 13:00:01.000000000", "symbol": "ORCL", "amount": 30}' |nc localhost 54300
// $ echo '{"ts": "2022-01-01 13:00:01.000000000", "symbol": "GOOGL", "amount": 50}' |nc localhost 54300
// $ echo '{"ts": "2022-01-01 13:00:02.000000000", "symbol": "ORCL", "amount": 40}' |nc localhost 54300
// $ echo '{"ts": "2022-01-01 13:00:05.000000000", "symbol": "GOOGL", "amount": 60}' |nc localhost 54300
// $ echo '{"ts": "2022-01-01 13:00:10.000000000", "symbol": "APPL", "amount": 100}' |nc localhost 54300

#include <assert.h>
#include <string.h>
#include <stdio.h>
#include <unistd.h>

#include <springql.h>

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

int main()
{
    SpringErrno ret;

    SpringConfig *config = spring_config_default();
    assert_not_null(config);

    SpringPipeline *pipeline = spring_open(config);
    assert_not_null(pipeline);

    ret = spring_command(
        pipeline,
        "CREATE SOURCE STREAM source_trade ("
        "    ts TIMESTAMP NOT NULL ROWTIME,"
        "    symbol TEXT NOT NULL,"
        "    amount INTEGER NOT NULL"
        ");");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK STREAM sink_avg_all ("
        "    ts TIMESTAMP NOT NULL ROWTIME,"
        "    avg_amount FLOAT NOT NULL"
        ");");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK STREAM sink_avg_by_symbol ("
        "    ts TIMESTAMP NOT NULL ROWTIME,"
        "    symbol TEXT NOT NULL,"
        "    avg_amount FLOAT NOT NULL"
        ");");
    assert_ok(ret);

    // Creates windows per 10 seconds ([:00, :10), [:10, :20), ...),
    // and calculate the average amount over the rows inside each window.
    //
    // Second parameter `DURATION_SECS(0)` means allowed latency for late data. You can ignore here.
    ret = spring_command(
        pipeline,
        "CREATE PUMP avg_all AS"
        "    INSERT INTO sink_avg_all (ts, avg_amount)"
        "    SELECT STREAM"
        "       FLOOR_TIME(source_trade.ts, DURATION_SECS(10)) AS min_ts,"
        "       AVG(source_trade.amount) AS avg_amount"
        "    FROM source_trade"
        "    GROUP BY min_ts"
        "    FIXED WINDOW DURATION_SECS(10), DURATION_SECS(0);");
    assert_ok(ret);

    // Creates windows per 2 seconds ([:00, :02), [:02, :04), ...),
    // and then group the rows inside each window having the same symbol.
    // Calculate the average amount for each group.
    ret = spring_command(
        pipeline,
        "CREATE PUMP avg_by_symbol AS"
        "    INSERT INTO sink_avg_by_symbol (ts, symbol, avg_amount)"
        "    SELECT STREAM"
        "       FLOOR_TIME(source_trade.ts, DURATION_SECS(2)) AS min_ts,"
        "       source_trade.symbol AS symbol,"
        "       AVG(source_trade.amount) AS avg_amount"
        "    FROM source_trade"
        "    GROUP BY min_ts, symbol"
        "    FIXED WINDOW DURATION_SECS(2), DURATION_SECS(0);");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK WRITER queue_avg_all FOR sink_avg_all"
        "    TYPE IN_MEMORY_QUEUE OPTIONS ("
        "        NAME 'q_avg_all'"
        "    );");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK WRITER queue_avg_by_symbol FOR sink_avg_by_symbol"
        "    TYPE IN_MEMORY_QUEUE OPTIONS ("
        "        NAME 'q_avg_by_symbol'"
        "    );");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SOURCE READER tcp_trade FOR source_trade"
        "    TYPE NET_SERVER OPTIONS ("
        "        PROTOCOL 'TCP',"
        "        PORT '54300'"
        "    );");
    assert_ok(ret);

    fprintf(stderr, "waiting JSON records in tcp/54300...\n");

    SpringSinkRow *row;
    bool is_err = false;
    while (1)
    {
#define TS_LEN 128
#define SYMBOL_LEN 6
        char ts[TS_LEN];
        char symbol[SYMBOL_LEN];

        // Fetching rows from q_avg_all.
        {
            row = spring_pop_non_blocking(pipeline, "q_avg_all", &is_err);
            if (row)
            {
                int r = spring_column_text(row, 0, (char *)ts, TS_LEN);
                assert((size_t)r == strlen(ts));

                float avg_amount;
                ret = spring_column_float(row, 1, &avg_amount);
                assert_ok(ret);

                fprintf(stderr, "[q_avg_all] %s\t%f\n", ts, avg_amount);
                spring_sink_row_close(row);
            }
            else
            {
                assert(!is_err);
            }
        }

        // Fetching rows from q_avg_by_symbol.
        row = spring_pop_non_blocking(pipeline, "q_avg_by_symbol", &is_err);
        if (row)
        {
            int r = spring_column_text(row, 0, (char *)ts, TS_LEN);
            assert((size_t)r == strlen(ts));

            r = spring_column_text(row, 1, (char *)symbol, SYMBOL_LEN);
            assert((size_t)r == strlen(symbol));

            float avg_amount;
            ret = spring_column_float(row, 2, &avg_amount);
            assert_ok(ret);

            fprintf(stderr, "[q_avg_by_symbol] %s\t%s\t%f\n", ts, symbol, avg_amount);
            spring_sink_row_close(row);
        }
        else
        {
            assert(!is_err);
        }

        // Avoid busy sleep.
        usleep(100000);
    }

    ret = spring_close(pipeline);
    assert_ok(ret);

    ret = spring_config_close(config);
    assert_ok(ret);

    return 0;
}
