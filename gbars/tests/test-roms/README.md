
```
arm-none-eabi-gcc build/crt0.s test1.c -mcpu=arm7tdmi -nostartfiles -Tbuild/lnkscript -o test1.gba
arm-none-eabi-gcc add.c -mcpu=arm7tdmi -nostartfiles -Tbuild/lnkscript -o add.gba
```
