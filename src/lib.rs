#![no_std]

type BOOL = i32;
type PBYTE = *mut u8;
type SIZE_T = isize;
type PVOID = *mut ();
type DWORD = u32;

#[link(name = "bcryptprimitives", kind = "raw-dylib")]
extern "system" {
    pub fn ProcessPrng(pbData: PBYTE, cbData: SIZE_T) -> BOOL;
}

#[link(name = "bcryptprimitives", kind = "raw-dylib")]
extern "system" {
    pub fn WaitOnAddress(
        Address: PVOID,
        CompareAddress: PVOID,
        AddressSize: SIZE_T,
        dwMilliseconds: DWORD,
    ) -> BOOL;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
