cmake_minimum_required(VERSION 3.16)

project(
  "crypto_crawler_ffi"
  LANGUAGES CXX
  VERSION 1.7.1
  DESCRIPTION "A rock-solid cryprocurrency crawler."
  HOMEPAGE_URL "https://github.com/soulmachine/crypto-crawler-ffi")

if(CMAKE_BUILD_TYPE STREQUAL "Release")
  set(CARGO_CMD cargo build --release)
  set(OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/release")
else()
  set(CARGO_CMD cargo build)
  set(OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/debug")
endif()

add_custom_target(
  crypto_crawler_ffi_cargo ALL
  COMMENT "Compiling ${PROJECT_NAME} cargo project"
  COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
  COMMAND cp "${OUTPUT_DIR}/libcrypto_crawler_ffi.a"
          "${CMAKE_CURRENT_BINARY_DIR}/libcrypto_crawler_ffi.a"
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

add_library(crypto_crawler_ffi STATIC IMPORTED GLOBAL)
add_dependencies(crypto_crawler_ffi crypto_crawler_ffi_cargo)
set_target_properties(
  crypto_crawler_ffi PROPERTIES IMPORTED_LOCATION
                            "${CMAKE_CURRENT_BINARY_DIR}/libcrypto_crawler_ffi.a")
file(GLOB HEADER_FILES include/*.h)
set_target_properties(crypto_crawler_ffi PROPERTIES PUBLIC_HEADER "${HEADER_FILES}")
add_library(crypto_crawler_ffi::crypto_crawler_ffi ALIAS crypto_crawler_ffi)

target_include_directories(crypto_crawler_ffi INTERFACE include/)
target_link_libraries(crypto_crawler_ffi INTERFACE pthread ssl crypto dl)
