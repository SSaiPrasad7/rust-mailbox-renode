set export

SINGLE_CORE_PLATFORM_PATH := "./platforms/core-v-mcu.repl"
SINGLE_CORE_RENODE_START_PATH := "./scripts/single-core-start.resc"
SINGLE_CORE_BINARY_FILE_PATH := "./rust/single-core/target/riscv32imac-unknown-none-elf/debug/rust"

DUAL_CORE_PLATFORM_PATH := "./platforms/dual-core-v-mcu.repl"
DUAL_CORE_RENODE_START_PATH := "./scripts/dual-core-start.resc"
CORE_0_BINARY_FILE_PATH := "./rust/dual-core/core0/target/riscv32imac-unknown-none-elf/debug/rust"
CORE_1_BINARY_FILE_PATH := "./rust/dual-core/core1/target/riscv32imac-unknown-none-elf/debug/rust"


default:
    just --help

single_core_build: 
    cd rust/single-core && cargo build
    
single_core_sections:
    size -Ax $SINGLE_CORE_BINARY_FILE_PATH

single_core_renode:
    renode --console -e "set bin @$SINGLE_CORE_BINARY_FILE_PATH; set platform_path @$SINGLE_CORE_PLATFORM_PATH; include @$SINGLE_CORE_RENODE_START_PATH"
    
single_core_debug:
    riscv32-unknown-elf-gdb -x ./scripts/connect.gdb $SINGLE_CORE_BINARY_FILE_PATH

dual_core_build: 
    cd rust/dual-core/core0 && cargo build
    cd rust/dual-core/core1 && cargo build

dual_core_sections:
    size -Ax $CORE_0_BINARY_FILE_PATH
    size -Ax $CORE_1_BINARY_FILE_PATH

dual_core_renode:
    renode --console -e "set bin0 @$CORE_0_BINARY_FILE_PATH; set bin1 @$CORE_1_BINARY_FILE_PATH; set platform_path @$DUAL_CORE_PLATFORM_PATH; include @$DUAL_CORE_RENODE_START_PATH"
    
dual_core_debug:
    riscv32-unknown-elf-gdb -x ./scripts/connect.gdb $CORE_0_BINARY_FILE_PATH