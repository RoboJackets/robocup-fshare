cmake_minimum_required(VERSION 3.0)

add_library(pid
    src/pid_tuner.cpp
    src/pid.cpp)

target_include_directories(pid PUBLIC
    "${CMAKE_CURRENT_SOURCE_DIR}/include")
