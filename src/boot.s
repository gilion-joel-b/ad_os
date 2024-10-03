.global _efi_main
_efi_main:
    ldr x30, =stack_top
    mov sp, x30
    bl kmain
    b .

