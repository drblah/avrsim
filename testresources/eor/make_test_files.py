file_contents = """
.nolist
.include "m328Pdef.inc"
.list


; start vector
.org 0x0000
        rjmp    main                    ; jump to main label

; main program
main:
{}
"""


instructions = ""

for Rd in range(0, 32):
    for Rr in range(0, 32):
        instructions += "\tEOR R{}, R{}\n".format(Rd, Rr)

f = open("eor.asm", "w")
f.write(file_contents.format(instructions))
f.close()