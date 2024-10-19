; bootloader.asm
BITS 16
ORG 0x7C00

start:
    ; Set up the stack
    xor ax, ax
    mov ss, ax
    mov sp, 0x7C00

    ; Print "Hello, World!"
    mov si, hello_msg

print_char:
    lodsb
    or al, al
    jz done
    mov ah, 0x0E
    int 0x10
    jmp print_char

done:
    ; Hang the system
    cli
    hlt

hello_msg db 'Hello, World!', 0

TIMES 510 - ($ - $$) db 0
DW 0xAA55
