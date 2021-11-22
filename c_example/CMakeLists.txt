cmake_minimum_required(VERSION 3.0.0)
project(springql-example VERSION 0.1.0)

add_library(springql_client SHARED IMPORTED)
add_executable(run_example example.c)
