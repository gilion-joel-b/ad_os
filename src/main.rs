#![no_std]
#![no_main]

mod uefi;
mod macros;
mod system_table;
mod linear_text_buffer;

use uefi::{ImageHandle, SystemTable};
use macros::entry;


#[no_mangle]
#[entry]
pub extern "efiapi" fn efi_main(image_handle: ImageHandle, system_table: SystemTableRefined) {

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
