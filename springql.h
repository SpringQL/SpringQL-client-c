// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

#ifndef _SPRINGQL_H_
#define _SPRINGQL_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Errno (error number) to be returned erroneous functions.
 */
typedef enum SpringErrno {
  Ok = 0,
  /**
   * Panic
   */
  Unknown = -1,
  ForeignIo = -2,
  ForeignSourceTimeout = -3,
  InputTimeout = -4,
  SpringQlCoreIo = -5,
  ThreadPoisoned = -6,
  InvalidOption = -7,
  InvalidFormat = -8,
  Unavailable = -9,
  Sql = -10,
  InvalidConfig = -11,
  Null = -12,
  Time = -13,
  /**
   * Insufficient buffer size
   */
  CInsufficient = -126,
  /**
   * Invalid null pointer
   */
  CNull = -127,
} SpringErrno;

/**
 * Configuration.
 */
typedef struct SpringConfig SpringConfig;

/**
 * Pipeline (dataflow definition) in SpringQL.
 */
typedef struct SpringPipeline SpringPipeline;

/**
 * Row object to pop from an in memory queue.
 */
typedef struct SpringSinkRow SpringSinkRow;

/**
 * Row object to push into an in memory queue.
 */
typedef struct SpringSourceRow SpringSourceRow;

/**
 * Builder of SpringSourceRow
 */
typedef struct SpringSourceRowBuilder SpringSourceRowBuilder;

/**
 * Returns default configuration.
 *
 * Returned value is not modifiable (it is just a void pointer).
 * If you would like to change the default configuration, use `spring_config_toml()` instead.
 */
struct SpringConfig *spring_config_default(void);

/**
 * Configuration by TOML format string.
 *
 * Returned value is not modifiable (it is just a void pointer).
 *
 * # Parameters
 *
 * - `overwrite_config_toml`: TOML format configuration to overwrite default.
 *   See https://springql.github.io/deployment/configuration for TOML format and configuration values.
 *
 * # Panics
 *
 * Currently, the process aborts when:
 *
 * - `overwrite_config_toml` includes invalid key and/or value.
 * - `overwrite_config_toml` is not valid as TOML.
 */
struct SpringConfig *spring_config_toml(const char *overwrite_config_toml);

/**
 * Frees heap occupied by a `SpringConfig`.
 *
 * # Returns
 *
 * - `Ok`: on success.
 * - `CNull`: `config` is a NULL pointer.
 */
enum SpringErrno spring_config_close(struct SpringConfig *config);

/**
 * Creates and open an in-process stream pipeline.
 *
 * # Returns
 *
 * - non-NULL: on success
 * - NULL: on failure. Check spring_last_err() for details.
 *
 * # Errors
 *
 * No errors are expected currently.
 */
struct SpringPipeline *spring_open(const struct SpringConfig *config);

/**
 * Frees heap occupied by a `SpringPipeline`.
 *
 * # Returns
 *
 * - `Ok`: on success.
 * - `CNull`: `pipeline` is a NULL pointer.
 */
enum SpringErrno spring_close(struct SpringPipeline *pipeline);

/**
 * Execute commands (DDL) to modify the pipeline.
 *
 * # Returns
 *
 * - `Ok`: on success.
 * - `Sql`:
 *   - Invalid SQL syntax.
 *   - Refers to undefined objects (streams, pumps, etc)
 *   - Other semantic errors.
 * - `InvalidOption`:
 *   - `OPTIONS` in `CREATE` statement includes invalid key or value.
 */
enum SpringErrno spring_command(const struct SpringPipeline *pipeline, const char *sql);

/**
 * Pop a row from an in memory queue. This is a blocking function.
 *
 * Do not call this function from threads.
 * If you need to pop from multiple in-memory queues using threads, use `spring_pop_non_blocking()`.
 * See: https://github.com/SpringQL/SpringQL/issues/125
 *
 * # Returns
 *
 * - non-NULL: on success
 * - NULL: on failure. Check spring_last_err() for details.
 *
 * # Errors
 *
 * - `Unavailable`: queue named `queue` does not exist.
 */
struct SpringSinkRow *spring_pop(const struct SpringPipeline *pipeline, const char *queue);

/**
 * Pop a row from an in memory queue. This is a non-blocking function.
 *
 * # Returns
 *
 * - non-NULL: Successfully get a row.
 * - NULL: Error occurred if `is_err` is true (check spring_last_err() for details). Otherwise, any row is not in the queue.
 *
 * # Errors
 *
 * - `Unavailable`: queue named `queue` does not exist.
 */
struct SpringSinkRow *spring_pop_non_blocking(const struct SpringPipeline *pipeline,
                                              const char *queue,
                                              bool *is_err);

/**
 * Push a row into an in memory queue. This is a non-blocking function.
 *
 * `row` is freed internally.
 *
 * # Returns
 *
 * - `Ok`: on success.
 * - `Unavailable`: queue named `queue` does not exist.
 */
enum SpringErrno spring_push(const struct SpringPipeline *pipeline,
                             const char *queue,
                             struct SpringSourceRow *row);

/**
 * Create a source row from JSON string
 *
 * # Returns
 *
 * - non-NULL: Successfully created a row.
 * - NULL: Error occurred.
 *
 * # Errors
 *
 * - `InvalidFormat`: JSON string is invalid.
 */
struct SpringSourceRow *spring_source_row_from_json(const char *json);

/**
 * Start creating a source row using a builder.
 *
 * # Returns
 *
 * Pointer to the builder
 */
struct SpringSourceRowBuilder *spring_source_row_builder(void);

/**
 * Add a BLOB column to the builder and return the new one.
 *
 * `builder` is freed internally.
 *
 * # Parameters
 *
 * - `builder`: Pointer to the builder created via spring_source_row_builder().
 * - `column_name`: Column name to add.
 * - `v`: BLOB value to add. The byte sequence is copied internally.
 * - `v_len`: `v`'s length.
 *
 * # Returns
 *
 * - non-NULL: Successfully created a row.
 * - NULL: Error occurred.
 *
 * # Errors
 *
 * - `Sql`: `column_name` is already added to the builder.
 */
struct SpringSourceRowBuilder *spring_source_row_add_column_blob(struct SpringSourceRowBuilder *builder,
                                                                 const char *column_name,
                                                                 const void *v,
                                                                 int v_len);

/**
 * Finish creating a source row using a builder.
 *
 * The heap space for the `builder` is internally freed.
 *
 * # Returns
 *
 * SpringSourceRow
 */
struct SpringSourceRow *spring_source_row_build(struct SpringSourceRowBuilder *builder);

/**
 * Frees heap occupied by a `SpringSinkRow`.
 *
 * # Returns
 *
 * - `Ok`: on success.
 * - `CNull`: `pipeline` is a NULL pointer.
 */
enum SpringErrno spring_sink_row_close(struct SpringSinkRow *row);

/**
 * Get a 2-byte integer column.
 *
 * # Parameters
 *
 * - `row`: A `SpringRow` pointer to get a column value from.
 * - `i_col`: The column index to get a value from.
 * - `out`: A pointer to a buffer to store the column value.
 *
 * # Returns
 *
 * - `Ok`: On success.
 * - `Unavailable`:
 *   - Column pointed by `i_col` is already fetched.
 *   - `i_col` is out of range.
 * - `CNull`: Column value is NULL.
 */
enum SpringErrno spring_column_short(const struct SpringSinkRow *row, uint16_t i_col, short *out);

/**
 * Get a 4-byte integer column.
 *
 * # Parameters
 *
 * - `row`: A `SpringRow` pointer to get a column value from.
 * - `i_col`: The column index to get a value from.
 * - `out`: A pointer to a buffer to store the column value.
 *
 * # Returns
 *
 * - `Ok`: On success.
 * - `Unavailable`:
 *   - Column pointed by `i_col` is already fetched.
 *   - `i_col` is out of range.
 * - `CNull`: Column value is NULL.
 */
enum SpringErrno spring_column_int(const struct SpringSinkRow *row, uint16_t i_col, int *out);

/**
 * Get an 8-byte integer column.
 *
 * # Parameters
 *
 * - `row`: A `SpringRow` pointer to get a column value from.
 * - `i_col`: The column index to get a value from.
 * - `out`: A pointer to a buffer to store the column value.
 *
 * # Returns
 *
 * - `Ok`: On success.
 * - `Unavailable`:
 *   - Column pointed by `i_col` is already fetched.
 *   - `i_col` is out of range.
 * - `CNull`: Column value is NULL.
 */
enum SpringErrno spring_column_long(const struct SpringSinkRow *row, uint16_t i_col, long *out);

/**
 * Get a 4-byte unsigned integer column.
 *
 * # Parameters
 *
 * - `row`: A `SpringRow` pointer to get a column value from.
 * - `i_col`: The column index to get a value from.
 * - `out`: A pointer to a buffer to store the column value.
 *
 * # Returns
 *
 * - `Ok`: On success.
 * - `Unavailable`:
 *   - Column pointed by `i_col` is already fetched.
 *   - `i_col` is out of range.
 * - `CNull`: Column value is NULL.
 */
enum SpringErrno spring_column_unsigned_int(const struct SpringSinkRow *row,
                                            uint16_t i_col,
                                            unsigned int *out);

/**
 * Get a text column.
 *
 * # Parameters
 *
 * - `row`: A `SpringRow` pointer to get a column value from.
 * - `i_col`: The column index to get a value from.
 * - `out`: A pointer to a buffer to store the column value.
 * - `out_len`: The length of the buffer pointed by `out`.
 *
 * # Returns
 *
 * - `> 0`: Length of the text.
 * - `Unavailable`:
 *   - Column pointed by `i_col` is already fetched.
 *   - `i_col` is out of range.
 * - `CNull`: Column value is NULL.
 */
int spring_column_text(const struct SpringSinkRow *row, uint16_t i_col, char *out, int out_len);

/**
 * Get a BLOB column.
 *
 * # Parameters
 *
 * - `row`: A `SpringRow` pointer to get a column value from.
 * - `i_col`: The column index to get a value from.
 * - `out`: A pointer to a buffer to store the column value.
 * - `out_len`: The length of the buffer pointed by `out`.
 *
 * # Returns
 *
 * - `> 0`: Length of the text.
 * - `Unavailable`:
 *   - Column pointed by `i_col` is already fetched.
 *   - `i_col` is out of range.
 * - `CNull`: Column value is NULL.
 */
int spring_column_blob(const struct SpringSinkRow *row, uint16_t i_col, void *out, int out_len);

/**
 * Get a bool column.
 *
 * # Parameters
 *
 * - `row`: A `SpringRow` pointer to get a column value from.
 * - `i_col`: The column index to get a value from.
 * - `out`: A pointer to a buffer to store the column value.
 *
 * # Returns
 *
 * - `Ok`: On success.
 * - `Unavailable`:
 *   - Column pointed by `i_col` is already fetched.
 *   - `i_col` is out of range.
 * - `CNull`: Column value is NULL.
 */
enum SpringErrno spring_column_bool(const struct SpringSinkRow *row, uint16_t i_col, bool *out);

/**
 * Get a 4-byte floating point column.
 *
 * # Parameters
 *
 * - `row`: A `SpringRow` pointer to get a column value from.
 * - `i_col`: The column index to get a value from.
 * - `out`: A pointer to a buffer to store the column value.
 *
 * # Returns
 *
 * - `Ok`: On success.
 * - `Unavailable`:
 *   - Column pointed by `i_col` is already fetched.
 *   - `i_col` is out of range.
 * - `CNull`: Column value is NULL.
 */
enum SpringErrno spring_column_float(const struct SpringSinkRow *row, uint16_t i_col, float *out);

/**
 * Write the most recent error number into `errno_` and message into a caller-provided buffer as a UTF-8
 * string, returning the number of bytes written.
 *
 * # Note
 *
 * This writes a **UTF-8** string into the buffer. Windows users may need to
 * convert it to a UTF-16 "unicode" afterwards.
 *
 * If there are no recent errors then this returns `0` (because we wrote 0
 * bytes). `-1` is returned if there are any errors, for example when passed a
 * null pointer or a buffer of insufficient size.
 *
 * # Returns
 *
 * - `0`: if there are no recent errors.
 * - `> 0`: the length of the recent error message.
 * - `< 0`: SpringErrno
 */
int spring_last_err(enum SpringErrno *errno_,
                    char *errmsg,
                    int errmsg_len);

/**
 * Calculate the number of bytes in the last error's error message **not**
 * including any trailing `null` characters.
 *
 * # Returns
 *
 * - `0`: if there are no recent errors.
 * - `> 0`: the length of the recent error message.
 */
int spring_last_errmsg_len(void);

#endif /* _SPRINGQL_H_ */
