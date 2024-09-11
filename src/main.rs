#![no_std]
#![no_main]

mod linear_text_buffer;
mod uefi;

use uefi::{ImageHandle, SystemTable};

#[no_mangle]
pub extern "efiapi" fn efi_main(image_handle: ImageHandle, system_table: SystemTable) {
    uefi::init_system_table(&system_table);
    println!("Hello, world!");
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
