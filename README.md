## SpringQL C Client

This repository is a C client for [SpringQL](https://gh01.base.toyota-tokyo.tech/SpringQL/SpringQL) and provides it provides:

- `springql.h`: C header file.
- `libspringql_client.{so,dylib}`: Shared library.

You can link your application with the shared library and run SpringQL in embedded mode.

### Getting Started

#### APIs

Take a short look to [springql.h](https://gh01.base.toyota-tokyo.tech/SpringQL/SpringQL-client-c/blob/main/springql.h), which declares all of C APIs and types to use from your application.

#### Installation

All you need to do are:

- Download latest header file and shared library from [release page](https://gh01.base.toyota-tokyo.tech/SpringQL/SpringQL-client-c/releases).
- Put `springql.h` to somewhere where your compiler recognize as an include path.
- Put `libspringql_client.{so,dylib}` to somewhere where your compiler recognize as a library path (one in `$LD_LIBRARY_PATH` is a good option).

#### Example application

See [`./c_example`](./c_example) for how to write and build a SpringQL embedded application.
