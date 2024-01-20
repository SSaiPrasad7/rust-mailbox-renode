set export

PLATFORM_PATH := "./platforms/core-v-mcu.repl"
BINARY_FILE_PATH := "./rust/target/riscv32imac-unknown-none-elf/debug/rust"
RENODE_START_PATH := "./scripts/start.resc"

default:
    just --help

build: 
    cd rust && cargo build
    
renode:
    renode --console -e "set bin @$BINARY_FILE_PATH; set platform_path @$PLATFORM_PATH; include @$RENODE_START_PATH"
    
debug:
    riscv32-unknown-elf-gdb -x ./scripts/connect.gdb $BINARY_FILE_PATH

sections:
    size -Ax $BINARY_FILE_PATH