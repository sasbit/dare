cmake_minimum_required(VERSION 3.14)
project(dare)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

include_directories(include)

find_package(OpenSSL REQUIRED)

add_executable(dare src/main.cpp)
target_link_libraries(dare OpenSSL::Crypto)

add_executable(tests tests/test_validation.cpp)
target_link_libraries(tests OpenSSL::Crypto)
