#![no_std]

pub use cobalt_boot::boot_fn as entrypoint;

use cobalt_boot::hook_panic_fn;
use core::{panic::PanicInfo};

#[doc(hidden)]
pub fn _panic(_: &PanicInfo) -> ! {
    loop {};
}