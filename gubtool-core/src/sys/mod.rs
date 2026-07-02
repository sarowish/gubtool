pub mod error;
pub mod pid;
pub use pid::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

use crate::{address::Address, attached::is_32, sys::error::ProcResult};
use assemble::AsmFolder;
use std::sync::LazyLock;

#[cfg(unix)]
static ASM_LIB32_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sys32.bin"));
static ASM_LIB64_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sys64.bin"));
#[cfg(unix)]
static ASM32: LazyLock<AsmFolder> = LazyLock::new(|| bincode::deserialize(ASM_LIB32_BYTES).unwrap());
static ASM64: LazyLock<AsmFolder> = LazyLock::new(|| bincode::deserialize(ASM_LIB64_BYTES).unwrap());

#[track_caller]
pub fn read_address_unsafe(address: impl Address) -> ProcResult<u64> {
    if is_32() {
        read_unsafe::<u32>(address).map(|addr| addr as u64)
    } else {
        read_unsafe::<u64>(address)
    }
}

pub fn print_asm_sizes() {
    println!("Core");
    ASM64.print_function_sizes();
    println!("\n");
}