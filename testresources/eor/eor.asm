.nolist
.include "m328Pdef.inc"
.list


; start vector
.org 0x0000
	rjmp	main			; jump to main label

; main program
main:
    EOR R0, R0
    ADD R0, R2
    ADD R3, R2
    