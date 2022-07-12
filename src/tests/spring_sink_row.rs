// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::ffi::CString;

use crate::*;

unsafe fn command(pipeline: *const SpringPipeline, sql: &str) {
    let sql = CString::new(sql).unwrap();
    let errno = spring_command(pipeline, sql.as_ptr());
    assert_eq!(errno, SpringErrno::Ok);
}

#[test]
fn test_spring_sink_row() {
    unsafe {
        let config = spring_config_default();
        assert!(!config.is_null());

        let pipeline = spring_open(config);
        assert!(!config.is_null());

        command(pipeline, "CREATE SOURCE STREAM source_1 (b BLOB NOT NULL);");
        command(pipeline, "CREATE SINK STREAM sink_1 (b BLOB NOT NULL);");
        command(
            pipeline,
            "
            CREATE PUMP pump_1 AS
                INSERT INTO sink_1 (b)
                SELECT STREAM source_1.b FROM source_1;
            ",
        );
        command(
            pipeline,
            "
            CREATE SINK WRITER queue_sink FOR sink_1
                TYPE IN_MEMORY_QUEUE OPTIONS (NAME 'q_sink');
            ",
        );
        command(
            pipeline,
            "
            CREATE SOURCE READER queue_src FOR source_1
                TYPE IN_MEMORY_QUEUE OPTIONS (NAME 'q_src');
            ",
        );

        let source_row = {
            let col = CString::new("b").unwrap();
            let val = vec![0x01u8, 0x02, 0x03];

            let builder = spring_source_row_builder();
            let builder = spring_source_row_add_column_blob(
                builder,
                col.as_ptr(),
                val.as_ptr().cast(),
                val.len().try_into().unwrap(),
            );
            assert!(!builder.is_null());

            spring_source_row_build(builder)
        };

        let q_src = CString::new("q_src").unwrap();
        let errno = spring_push(pipeline, q_src.as_ptr(), source_row);
        assert_eq!(errno, SpringErrno::Ok);

        let q_sink = CString::new("q_sink").unwrap();
        let sink_row = spring_pop(pipeline, q_sink.as_ptr());

        spring_sink_row_close(sink_row);

        spring_close(pipeline);
        spring_config_close(config);
    }
}
