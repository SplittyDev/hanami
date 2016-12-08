ENTRY(start)

SECTIONS {
  . = 1M;

  .rodata ALIGN(0x1000) : {
    KEEP(*(.multiboot))
    *(.rodata .rodata.*)
  }

  .text ALIGN(0x1000) : {
    *(.text .text.*)
  }

  .data ALIGN(0x1000) : {
    *(.data .data.*)
  }

  .bss ALIGN(0x1000) : {
    *(.bss)
  }
}