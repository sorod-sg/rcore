OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x802000000;  

SECTIONS   
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
       *(.rodata .rodata.*)
    }

    . = ALIGN(4k);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
    }
    . = ALIGN(4k);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
