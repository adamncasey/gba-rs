
CFLAGS=-mcpu=arm7tdmi -std=c99

combined-test: test_add test_multiply test_alignment
	arm-none-eabi-gcc test-main.c test_add.o test_multiply.o test_alignment.o $(CFLAGS) -nostartfiles -Tbuild/lnkscript -o combined.elf
	arm-none-eabi-objcopy -O binary combined.elf combined.gba

test_add:
	arm-none-eabi-gcc -c test_add.c $(CFLAGS)

test_multiply:
	arm-none-eabi-gcc -c test_multiply.c $(CFLAGS)

test_alignment:
	arm-none-eabi-gcc -c test_alignment.c $(CFLAGS)
