#![no_std]
#![no_main]

mod uefi;
use uefi::{ImageHandle, SystemTable};

#[no_mangle]
pub extern "efiapi" fn efi_main(image_handle: ImageHandle, system_table: *const SystemTable) {

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
