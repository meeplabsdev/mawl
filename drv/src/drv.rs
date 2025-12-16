#![no_std]
#![cfg(not(test))]
use common::{
    types::types::P_UNICODE_STRING,
    utils::{logging::log, ntoskrnl::KernelAlloc},
};

#[unsafe(no_mangle)]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 {
    unimplemented!()
}

#[global_allocator]
static ALLOCATOR: KernelAlloc = KernelAlloc;

#[unsafe(export_name = "_fltused")]
static _FLTUSED: i32 = 0;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "system" fn driver_entry(
    DriverObject: P_DRIVER_OBJECT,
    RegistryPath: P_UNICODE_STRING,
) -> u32 {
    log("this is a test");
    return 0;
}
