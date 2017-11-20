# Examples (uncomment one)
EXAMPLE      := blinky_delay
#EXAMPLE      := blinky_clint
#EXAMPLE      := blinky_pwm
#EXAMPLE      := blinky_plic
#EXAMPLE      := hello_world
#EXAMPLE      := panicking

# Board crate (uncomment one)
BOARD        := hifive

TARGET       := riscv32-unknown-none
TARGET_DIR   := $(abspath ./target/$(TARGET)/debug)
EXAMPLE_BIN  := $(TARGET_DIR)/examples/$(EXAMPLE)
OPENOCD_CFG  := $(wildcard $(TARGET_DIR)/build/$(BOARD)-*/out/openocd.cfg)

build:
	xargo build --examples --target $(TARGET) $(ARGS)

test:
	xargo test --all --target $(TARGET) $(ARGS)

clean:
	xargo clean $(ARGS)

readelf:
	llvm-readelf -a -h -s -r -symbols $(EXAMPLE_BIN) $(ARGS)

objdump:
	llvm-objdump -d -S $(EXAMPLE_BIN) $(ARGS)

size:
	llvm-size $(EXAMPLE_BIN) $(ARGS)

# .gdbinit adds a upload command to gdb
gdb:
	riscv32-unknown-elf-gdb $(EXAMPLE_BIN) $(ARGS)

openocd:
	openocd -f $(OPENOCD_CFG) $(ARGS)

upload:
	openocd -f $(OPENOCD_CFG) \
		-c "flash protect 0 64 last off; program ${EXAMPLE_BIN}; resume 0x20400000; exit"

framedump:
	riscv32-unknown-elf-readelf --debug-dump=frames $(EXAMPLE_BIN) $(ARGS)

.PHONY: build clean readelf objdump framedump size gdb openocd spike
