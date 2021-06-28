global install_gdt
install_gdt:
    lgdt [rdi]
    push rbp
    mov rbp, rsp ;; save stack

    push qword 16 ;; kernel data
    push rbp ;; save stack
    pushf ;; flags
    push qword 8 ;; kernel code
    push .trampoline ;; return point

    iretq ;; interrupt return
.trampoline
    pop rbp

    ;; All point to data segment
    mov ax, 16
    mov ds, ax
    mov es, ax
    mov ss, ax
    ret
