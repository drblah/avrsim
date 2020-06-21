ASFLAGS=-I ../include
TARGET=eor

SRC = eor.asm

all: ${TARGET}.dissasm

disassemble: ${TARGET}.hex
	vavrdisasm ${TARGET}.hex > ${TARGET}.dissasm

${TARGET}.hex: ${SRC}
	avra ${ASFLAGS} -o ${TARGET}.hex ${SRC}

${SRC}:
	python3 make_test_files.py 
