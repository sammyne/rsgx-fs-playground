cmake_minimum_required(VERSION 3.10)

# must
set(rtsLib sgx_urts${enclaveLibSuffix})

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${sgxFlags} \
    -fPIC -Wno-attributes")

set(bridgeImpl ${CMAKE_CURRENT_BINARY_DIR}/enclave_u.c) 
set(bridge ${CMAKE_CURRENT_BINARY_DIR}/enclave_u.h ${bridgeImpl})

set(uBridgeLib ${PROJECT_NAME}-enclave_u)
set(uBridgeLibPath ${CMAKE_CURRENT_BINARY_DIR})

include_directories(
    ${rsgxPath}/edl
    ${sgxPath}/include 
    ${CMAKE_CURRENT_BINARY_DIR}
)

add_custom_command(
    OUTPUT ${bridge}
    COMMAND ${sgxEdger8r} --untrusted ${edl} 
       --search-path ${enclavePath} --search-path ${sgxPath}/include
       --search-path ${rsgxPath}/edl
    COMMENT "[GEN] => ${bridge}"
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR})

add_library(${uBridgeLib} STATIC ${bridgeImpl})

set(appBin ${CMAKE_CURRENT_BINARY_DIR}/cargo/release/app)
set(benchmarkBin ${CMAKE_CURRENT_BINARY_DIR}/cargo/release/benchmark)

# redirect target directory into ${CMAKE_CURRENT_BINARY_DIR}/cargo to avoid mess up the source
# directory
add_custom_target(${app} ALL
    COMMAND SGX_SDK=${sgxPath} SGX_MODE=${SGX_MODE} BRIDGE_LIB_PATH=${uBridgeLibPath} 
        BRIDGE_LIB_NAME=${uBridgeLib} cargo build --release 
        --target-dir=${CMAKE_CURRENT_BINARY_DIR}/cargo
    COMMENT "[CARGO] app => ${appBin}"
    DEPENDS ${uBridgeLib}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

set(dataPath "${PROJECT_SOURCE_DIR}/testdata/10-160000.csv")

add_custom_target(example
    ${appBin} ${CMAKE_CURRENT_BINARY_DIR}/../enclave/${enclaveLib} ${dataPath}
    COMMENT "[RUN] ${PROJECT_NAME} ..."
    DEPENDS ${app} ${enclave}
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
)

add_custom_target(format-${app}
    cargo fmt
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)
