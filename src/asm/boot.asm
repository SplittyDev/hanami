section .multiboot

;
; Multiboot2 constants.
;
MB_MAGIC    equ 0xe85250d6
MB_ARCH     equ 0
MB_TYPE     equ 0
MB_SIZE     equ 0
MB_FLAGS    equ 0
MB_CHKSUM   equ MB_CHKBNDS - MB_CHKRVAL
MB_LENGTH   equ multiboot.end - multiboot.start
MB_CHKRVAL  equ MB_MAGIC + MB_ARCH + MB_LENGTH
MB_CHKBNDS  equ 1 << 32

;
; The actual multiboot2 header.
;
multiboot:
.start:
    dd MB_MAGIC
    dd MB_ARCH
    dd MB_LENGTH
    dd MB_CHKSUM
    dd MB_TYPE
    dd MB_FLAGS
    dd MB_SIZE
.end:

section .text
global start
extern kmain
bits 32

;
; This is where GRUB takes us.
;
start:
    cli
    mov esp, stack.top
    mov edi, ebx

;
; Paging magic.
;
setup_paging:
;
; Links the page tables together.
;
.link:
    %macro makelink 2
    mov eax, page_tables.%1
    or eax, 0x03
    mov dword [page_tables.%2], eax
    %endmacro
    makelink p3, p4
    makelink p2, p3
    mov ecx, 0
;
; Maps the p2 table.
;
.map:
    mov eax, 0x200000
    mul ecx
    or eax, 0x83
    mov [page_tables.p2 + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .map
;
; Loads the p4 table into cr3.
;
.load:
    mov eax, page_tables.p4
    mov cr3, eax
;
; Enables PAE (Physical Address Extension).
;
.enable_pae:
    mov eax, cr4
    or eax, 0x20
    mov cr4, eax
;
; Sets the longmode bit.
;
.enable_longmode:
    mov ecx, 0xC0000080
    rdmsr
    or eax, 0x100
    wrmsr
;
; Enables paging.
;
.enable_paging:
    mov eax, cr0
    or eax, 0x80010000
    mov cr0, eax

;
; GDT magic.
;
setup_gdt:
    lgdt [gdt64.ptr]
    mov ax, gdt64._data
    mov ds, ax
    mov es, ax
    mov ss, ax
    jmp gdt64._code:kmain

section .rodata
;
; GDT register.
;
gdt64:
%define NULL        0
%define READWRITE   1 << 0x29
%define EXECUTABLE  1 << 0x2B
%define CODEDATA    1 << 0x2C
%define PRESENT     1 << 0x2F
%define BITS64      1 << 0x35
;
; Null descriptor.
;
._null:
	dq NULL

;
; Code descriptor.
;
._code: equ $ - gdt64
    dq NULL \
    | READWRITE \
    | EXECUTABLE \
    | CODEDATA \
    | PRESENT \
    | BITS64

;
; Data descriptor.
;
._data: equ $ - gdt64
    dq NULL \
    | READWRITE \
    | CODEDATA \
    | PRESENT
.ptr:
    ;
    ; Limit.
    ;
    dw .ptr - gdt64 - 1
    ;
    ; Base.
    ;
	dq gdt64

section .bss
align 4096
page_tables:
.p4:
    resb 4096
.p3:
    resb 4096
.p2:
    resb 4096
stack:
.bottom:
    resb 4096
.top: