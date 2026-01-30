bits 16
org 0x7c00

start:
    mov si, msg
.print:
    lodsb
    or al, al
    jz hang
    mov ah, 0x0e
    int 0x10
    jmp .print

hang:
    jmp $

msg db "Welcome to Hello World OS", 0

times 510-($-$$) db 0
dw 0xaa55
