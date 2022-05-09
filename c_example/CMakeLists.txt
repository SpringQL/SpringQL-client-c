cmake_minimum_required(VERSION 3.0.0)
project(springql-example VERSION 0.3.0)

add_compile_options(-Wall -Wextra -pedantic -Werror)

include_directories(..)
link_directories(../target/debug)

add_executable(run_example example.c)
target_link_libraries(run_example springql_client)
