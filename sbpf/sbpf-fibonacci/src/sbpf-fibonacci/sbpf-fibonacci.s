.globl entrypoint

.equ INPUT_DATA_OFFSET, 0x10
entrypoint:
    ldxdw r4, [r1+INPUT_DATA_OFFSET]
    jle r4, 0x02, print_one

    mov64 r5, 0x01
    mov64 r6, 0x01
    mov64 r3, 0x02

loop:
    add64 r7, r5, r6
    mov64 r5, r6
    mov64 r6, r7
    add64 r3, 0x01
    jle r3, r4, loop


print_result:
    mov64 r1, r6
    xor64 r2, r2
    xor64 r3, r3
    xor64 r4, r4
    xor64 r5, r5
    call sol_log_64_
    exit
print_one:
    mov64 r1, 0x01
    call sol_log_64_
    exit

.extern sol_log_64_
