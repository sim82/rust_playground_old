cmake_minimum_required(VERSION 2.8)
project(test NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}")

# Get the dependencies of all the crates
get_rust_deps(meh.rs MEH_DEPS)

# Build the library
rust_crate(meh.rs
           TARGET_NAME MEH
           DEPENDS "${MEH_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type dylib )

add_custom_target(library_target
                  ALL
                  DEPENDS ${TESTLIB_FULL_TARGET})

# Build examples
rust_crate(disk_hash.rs
           TARGET_NAME DISK_HASH
           DEPENDS "${DISK_HASH_FULL_TARGET}")

add_custom_target(examples_target
                  ALL
                  DEPENDS ${MEH_FULL_TARGET} ${DISK_HASH_FULL_TARGET})

