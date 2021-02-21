gdtr dw 0
     dd 0

section .text
global set_gdt:function (set_gdt.end - set_gdt)
set_gdt:
    mov  eax       , [esp + 4]
    mov  [gdtr + 2], eax
    mov  ax        , [ESP + 8]
    mov  [gdtr]    , ax
    lgdt [gdtr]
    ret
.end:

reload_segments:
    ; Reload code selector
    jmp 0x08:reload_cs
reload_cs: ; Reload data segments
    mov ax, 0x10 ; New data selector
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret
