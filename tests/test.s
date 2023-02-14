; Sample code for a bootloader, should be usable for 


[BITS 16]


[SECTION .pre_move vstart=0x7c00 align=1]

_start:
    cli

    xor ax, ax
    mov ds, ax
    mov es, ax

    ;set up stack
    mov ss, ax
    mov sp, 0x7c00


.copy_lower:
    cld
    mov cx, 0x0100 ;copy 0x100 words(512 bytes)
    mov si, 0x7c00 ;src is 0x7c00
    mov di, 0x0500 ;copy to 0x500
    rep movsw

    jmp 0x0:low_start

pre_move_len equ $-$$
[SECTION .post_move_code vstart=0x500+pre_move_len align=1]
low_start:
    sti
    push dx

    mov bx, PT1
    mov cx, 4
    .CKPTloop:
        mov al, byte [bx]
        test al, 0x80
        jnz .partition_found
        add bx, 0x10
        dec cx
        jnz .CKPTloop
        mov al, 'n'
        jmp _fail


    .partition_found:




_fail:
    mov bx, 0xb800
    mov es, bx
    mov byte [es:0x0], al
.rep:
    hlt
    jmp _fail.rep


[SECTION parition start=0x1b4]
UID times 10 db 0
PT1 times 16 db 0
PT2 times 16 db 0
PT3 times 16 db 0
PT4 times 16 db 0