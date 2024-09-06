#![no_std]
#![no_main]

#[no_mangle]
pub extern "efiapi" fn efi_main() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
