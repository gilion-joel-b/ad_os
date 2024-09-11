use core::{ptr, sync::atomic::{AtomicPtr, Ordering::SeqCst}};

#[repr(C)]
pub struct SystemTable {
    header: [u8; 24],
    firmware_vendor: u64,
    firmware_revision: u32,
    input_handle: ImageHandle,
    input: u64,
    output_handle: ImageHandle,
    pub output: *const TextOutputProtocol,
    error_handle: ImageHandle,
    error: u64,
    runtime: u64,
    boot: u64,
    no_of_entries: usize,
    config_table: u64,
}

#[repr(C)]
pub struct TextOutputProtocol {
    reset: u64,
    pub output_string: OutputString,
    test_output: u64,
    query_mode: u64,
    set_mode: u64,
    set_attribute: u64,
    pub clear_screen: u64,
    set_cursor_position: u64,
    enable_cursor: u64,
    mode: u64,
}
pub type OutputString =
    extern "efiapi" fn(output_protocol: *const TextOutputProtocol, string: *const u16) -> Status;
pub type Status = usize;

pub type ImageHandle = u64;

static SYSTEM_TABLE: AtomicPtr<SystemTable> = AtomicPtr::new(ptr::null_mut());

pub fn init_system_table(system_table: *const SystemTable) {
    SYSTEM_TABLE.store(system_table.cast_mut(), SeqCst);
}

pub fn system_table() -> &'static SystemTable {
    unsafe { SYSTEM_TABLE.load(SeqCst).as_ref().unwrap() }
}
