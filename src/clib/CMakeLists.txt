cmake_minimum_required(VERSION 3.17)
project(clib C)

set(CMAKE_C_STANDARD 11)
set(LIBRARY_OUTPUT_PATH ${CMAKE_SOURCE_DIR}/lib)

add_library(clib SHARED library.c library.h)