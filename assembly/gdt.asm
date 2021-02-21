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