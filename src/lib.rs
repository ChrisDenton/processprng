#![no_std]

type BOOL = i32;
type PBYTE = *mut u8;
type SIZE_T = isize;

#[link(name = "bcryptprimitives", kind = "raw-dylib")]
extern "system" {
    pub fn ProcessPrng(pbData: PBYTE, cbData: SIZE_T) -> BOOL;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
