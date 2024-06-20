set export

SINGLE_CORE_PLATFORM_PATH := "./platforms/core-v-mcu.repl"
SINGLE_CORE_RENODE_START_PATH := "./scripts/single-core-start.resc"
SINGLE_CORE_BINARY_FILE_PATH := "./rust/target/riscv32imac-unknown-none-elf/debug/single-core"

DUAL_CORE_PLATFORM_PATH := "./platforms/dual-core-v-mcu.repl"
DUAL_CORE_RENODE_START_PATH := "./scripts/dual-core-start.resc"
CORE_0_BINARY_FILE_PATH := "./rust/target/riscv32imac-unknown-none-elf/debug/core0"
CORE_1_BINARY_FILE_PATH := "./rust/target/riscv32imac-unknown-none-elf/debug/core1"


default:
    just --help

build: 
    cd rust && cargo build
    
single-core-renode:
    renode --console -e "set bin @$SINGLE_CORE_BINARY_FILE_PATH; set platform_path @$SINGLE_CORE_PLATFORM_PATH; include @$SINGLE_CORE_RENODE_START_PATH"

dual-core-renode:
    renode --console -e "set bin0 @$CORE_0_BINARY_FILE_PATH; set bin1 @$CORE_1_BINARY_FILE_PATH; set platform_path @$DUAL_CORE_PLATFORM_PATH; include @$DUAL_CORE_RENODE_START_PATH"
    
single-core-sections:
    size -Ax $SINGLE_CORE_BINARY_FILE_PATH

dual-core-sections:
    size -Ax $CORE_0_BINARY_FILE_PATH
    size -Ax $CORE_1_BINARY_FILE_PATH

single-core-debug:
    riscv32-unknown-elf-gdb -x ./scripts/connect.gdb $SINGLE_CORE_BINARY_FILE_PATH
    
dual-core-debug:
    riscv32-unknown-elf-gdb -x ./scripts/connect.gdb $CORE_0_BINARY_FILE_PATH