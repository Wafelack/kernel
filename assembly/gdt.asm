global set_gdt
set_gdt:
    mov eax, [esp + 4]
    lgdt [eax]
    ret

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
