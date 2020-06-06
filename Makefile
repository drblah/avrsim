hex: testprogram.s testprogram.bin
	avr-objcopy -j .text -j.data -O ihex testprogram.bin testprogram.hex

bin_asm: testprogram.s testprogram.bin

testprogram.s: testprogram.c
	avr-gcc -S -mmcu=atmega328p -o testprogram.s testprogram.c

testprogram.bin: testprogram.c
	avr-gcc -mmcu=atmega328p -o testprogram.bin testprogram.c