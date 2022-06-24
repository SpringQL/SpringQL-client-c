// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

// Usage:
//
// $ ./a.out  # waiting for connection...
// $ echo '{"ts": "2022-01-01 13:00:00.000000000", "temperature": 5.3}' |nc localhost 54300

#include <assert.h>
#include <string.h>
#include <stdio.h>

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
        "CREATE SOURCE STREAM source_temperature_celsius ("
        "    ts TIMESTAMP NOT NULL ROWTIME,"
        "    temperature FLOAT NOT NULL"
        ");");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK STREAM sink_temperature_fahrenheit ("
        "    ts TIMESTAMP NOT NULL ROWTIME,"
        "    temperature FLOAT NOT NULL"
        ");");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE PUMP c_to_f AS"
        "    INSERT INTO sink_temperature_fahrenheit (ts, temperature)"
        "    SELECT STREAM"
        "       source_temperature_celsius.ts,"
        "       32.0 + source_temperature_celsius.temperature * 1.8"
        "    FROM source_temperature_celsius;");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SINK WRITER queue_temperature_fahrenheit FOR sink_temperature_fahrenheit"
        "    TYPE IN_MEMORY_QUEUE OPTIONS ("
        "        NAME 'q'"
        "    );");
    assert_ok(ret);

    ret = spring_command(
        pipeline,
        "CREATE SOURCE READER tcp_temperature_celsius FOR source_temperature_celsius"
        "    TYPE NET_SERVER OPTIONS ("
        "        PROTOCOL 'TCP',"
        "        PORT '54300'"
        "    );");
    assert_ok(ret);

    fprintf(stderr, "waiting JSON records in tcp/54300...\n");

    SpringSinkRow *row;
    while (1)
    {
        row = spring_pop(pipeline, "q");
        assert_not_null(row);

#define TS_LEN 128
        const char ts[TS_LEN];

        int r = spring_column_text(row, 0, (char *)ts, TS_LEN);
        assert((size_t)r == strlen(ts));

        float temperature_fahrenheit;
        ret = spring_column_float(row, 1, &temperature_fahrenheit);
        assert_ok(ret);

        fprintf(stderr, "%s\t%f\n", ts, temperature_fahrenheit);
        spring_sink_row_close(row);
    }

    ret = spring_close(pipeline);
    assert_ok(ret);

    ret = spring_config_close(config);
    assert_ok(ret);

    return 0;
}
