cmake_minimum_required(VERSION 3.0.0)
project(springql-example VERSION 0.3.0)

add_compile_options(-Wall -Wextra -pedantic -Werror)

include_directories(..)
link_directories(../target/debug)

add_executable(run_trade_projection trade_projection/trade_projection.c)
target_link_libraries(run_trade_projection springql_client)

add_executable(run_doc_app1 doc_app1/doc_app1.c)
target_link_libraries(run_doc_app1 springql_client)

add_executable(run_doc_app2 doc_app2/doc_app2.c)
target_link_libraries(run_doc_app2 springql_client)
