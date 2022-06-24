# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][Keep a Changelog] and this project adheres to [Semantic Versioning][Semantic Versioning].

We originally added `### For developers` sections in order to separate changes for end-users and developers.
All other sections are for end-users.

The CHANGELOG mainly includes changes only applied to the C client.
Also check the changes in springql-core: <https://github.com/SpringQL/SpringQL/blob/main/CHANGELOG.md>.

<!-- markdownlint-disable MD024 -->
## [Unreleased]

## [v0.14.0] - 2022-06-24

### Changed

- `SpringRow` into `SpringSinkRow` ([#50](https://github.com/SpringQL/SpringQL-client-c/pull/50))
- `spring_row_close` -> `spring_sink_row_close` ([#50](https://github.com/SpringQL/SpringQL-client-c/pull/50))

### Added

- Following new APIs: ([#50](https://github.com/SpringQL/SpringQL-client-c/pull/50))
  - `SpringSourceRow`
  - `spring_source_row_from_json`
  - `spring_source_row_close`

## [v0.13.0+4]

### Fixed

- `spring_column_blob()` mistakenly adds trailing null-termination to dest buffer. ([#48](https://github.com/SpringQL/SpringQL-client-c/pull/48))

## [v0.13.0+3]

### Fixed

- `spring_column_blob()`'s `out_len` check. ([#47](https://github.com/SpringQL/SpringQL-client-c/pull/47))

## [v0.13.0+2]

### Added

- add include guard to `springql.h`. ([#46](https://github.com/SpringQL/SpringQL-client-c/pull/46))

## [v0.13.0]

### Added

- `spring_column_blob()` and `spring_column_unsigned_int()`. ([#45](https://github.com/SpringQL/SpringQL-client-c/pull/45))
- `SpringErrno::Time` error number. ([#45](https://github.com/SpringQL/SpringQL-client-c/pull/45))

## [v0.12.0]

### For developers

- refactor: stop calling spring_config_default(). ([#43](https://github.com/SpringQL/SpringQL-client-c/pull/43))

## [v0.11.0]

### For developers

- Rewrite with using springql-core's new APIs. ([#36](https://github.com/SpringQL/SpringQL-client-c/pull/36))

## [v0.9.0+2]

Depends on springql-core v0.9.0.

### Added

- Better doc comments on the header file ([#35](https://github.com/SpringQL/SpringQL-client-c/pull/35))

## [v0.9.0]

Depends on springql-core v0.9.0.

## [v0.8.0]

Depends on springql-core v0.8.0.

## [v0.7.1]

Started to write CHANGELOG from this version.

Depends on springql-core v0.7.1.

---

<!-- Links -->
[Keep a Changelog]: https://keepachangelog.com/
[Semantic Versioning]: https://semver.org/

<!-- Versions -->
[Unreleased]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.13.0+4...HEAD
[Released]: https://github.com/SpringQL/SpringQL-client-c/releases
[v0.13.0+4]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.13.0+2...v0.13.0+4
[v0.13.0+3]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.13.0+2...v0.13.0+3
[v0.13.0+2]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.13.0...v0.13.0+2
[v0.13.0]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.12.0...v0.13.0
[v0.12.0]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.11.0...v0.12.0
[v0.11.0]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.9.0+2...v0.11.0
[v0.9.0+2]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.9.0...v0.9.0+2
[v0.9.0]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.8.0...v0.9.0
[v0.8.0]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.7.1...v0.8.0
[v0.7.1]: https://github.com/SpringQL/SpringQL-client-c/compare/v0.7.0...v0.7.1
