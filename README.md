## SpringQL C Client

This repository is a C client for [SpringQL](https://github.com/SpringQL/SpringQL) and provides it provides:

- `springql.h`: C header file.
- `libspringql_client.{so,dylib}`: Shared library.

You can link your application with the shared library and run SpringQL in embedded mode.

## Documentation

Read <https://SpringQL.github.io/> for installation guide, tutorials, and references.

## Versioning

[Semantic versioning](https://semver.org/) in `<major>.<minor>.<patch>(\+<build>)?` format.

`<major>.<minor>.<patch>` is exactly the same as the version of `springql-core` crate.

`<build>` is an incremental number. `N`th build (`N > 1`) for the `springql-core vX.Y.Z` is `vX.Y.Z+N`, for example.

## Development

### Build

```bash
cargo build
```

to generate `springql.h` and `target/debug/libspringql_client.{so,dylib}`.

### Deployment

Create & push git tag named `v*` to release the header file and shared libraries to GitHub Releases.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in SpringQL by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Copyright (c) 2022 TOYOTA MOTOR CORPORATION.
