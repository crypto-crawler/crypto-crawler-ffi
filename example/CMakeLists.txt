cmake_minimum_required (VERSION 3.16)
include(FetchContent)

project (example CXX)

FetchContent_Declare(
    crypto_crawler
    GIT_REPOSITORY https://github.com/soulmachine/crypto-crawler-ffi
    GIT_TAG main)
FetchContent_MakeAvailable(crypto_crawler)
include_directories(${crypto_crawler_SOURCE_DIR}/include)

add_executable (example example.cpp)

#target_link_libraries (example pthread ssl crypto dl crypto_crawler)
target_link_libraries (example crypto_crawler)
