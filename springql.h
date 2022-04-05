// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Errno (error number) to be returned erroneous functions.
 *
 * See springql_core::api::error::SpringError for details of each error reason.
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
  /**
   * Insufficient buffer size
   */
  CInsufficient = -126,
  /**
   * Invalid null pointer
   */
  CNull = -127,
} SpringErrno;

typedef void *SpringConfig;

typedef void *SpringPipeline;

typedef void *SpringRow;

/**
 * See: springql_core::api::spring_config_default
 *
 * Returned value is not modifiable (it is just a void pointer).
 * If you would like to change the default configuration, use `spring_config_toml()` instead.
 */
SpringConfig *spring_config_default(void);

/**
 * See: springql_core::api::spring_config_default
 *
 * Returned value is not modifiable (it is just a void pointer).
 * If you would like to change the default configuration, use `spring_config_toml()` instead.
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
SpringConfig *spring_config_toml(const char *overwrite_config_toml);

/**
 * # Returns
 *
 * - `0`: if there are no recent errors.
 * - `< 0`: SpringErrno
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
enum SpringErrno spring_config_close(SpringConfig *config);

/**
 * See: springql_core::api::spring_open
 *
 * # Returns
 *
 * - non-NULL: on success
 * - NULL: on failure. Check spring_last_err() for details.
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
SpringPipeline *spring_open(const SpringConfig *config);

/**
 * # Returns
 *
 * - `0`: if there are no recent errors.
 * - `< 0`: SpringErrno
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
enum SpringErrno spring_close(SpringPipeline *pipeline);

/**
 * See: springql_core::api::spring_command
 *
 * # Returns
 *
 * - `0`: if there are no recent errors.
 * - `< 0`: SpringErrno
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
enum SpringErrno spring_command(const SpringPipeline *pipeline, const char *sql);

/**
 * See: springql_core::api::spring_pop
 *
 * # Returns
 *
 * - non-NULL: on success
 * - NULL: on failure. Check spring_last_err() for details.
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
SpringRow *spring_pop(const SpringPipeline *pipeline, const char *queue);

/**
 * # Returns
 *
 * - `0`: if there are no recent errors.
 * - `< 0`: SpringErrno
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
enum SpringErrno spring_row_close(SpringRow *row);

/**
 * See: springql_core::api::spring_column_i32
 *
 * # Returns
 *
 * - `0`: if there are no recent errors.
 * - `< 0`: SpringErrno
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
enum SpringErrno spring_column_int(const SpringRow *row, uint16_t i_col, int *out);

/**
 * See: springql_core::api::spring_column_text
 *
 * This returns UTF-8 string into `out`.
 *
 * # Returns
 *
 * - `0`: if there are no recent errors.
 * - `> 0`: the length of the recent error message.
 * - `< 0`: SpringErrno
 *
 * # Safety
 *
 * This function is unsafe because it uses raw pointer.
 */
int spring_column_text(const SpringRow *row, uint16_t i_col, char *out, int out_len);

/**
 * Write the most recent error number into `errno` and message into a caller-provided buffer as a UTF-8
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
 *
 * # Safety
 *
 * This function is unsafe because it writes into a caller-provided buffer.
 */
int spring_last_err(enum SpringErrno *errno,
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
