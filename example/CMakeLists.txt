cmake_minimum_required (VERSION 3.16)
include(FetchContent)

project (example CXX)

FetchContent_Declare(
    crypto_crawler_ffi
    GIT_REPOSITORY https://github.com/soulmachine/crypto-crawler-ffi
    GIT_TAG main)
FetchContent_MakeAvailable(crypto_crawler_ffi)
include_directories(${crypto_crawler_ffi_SOURCE_DIR}/include)

add_executable (example example.cpp)

target_link_libraries (example PRIVATE crypto_crawler_ffi)
