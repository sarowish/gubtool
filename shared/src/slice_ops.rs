use anyhow::{Result, anyhow, bail};
use pelite::Pod;

#[track_caller]
pub fn read_from_slice<T: Pod>(array: &[u8], offset: u64) -> Result<T> {
    let file_location = std::panic::Location::caller();
    let offset = offset as usize;
    let size = std::mem::size_of::<T>();
    let end = offset.checked_add(size)
        .ok_or_else(|| anyhow::anyhow!("{}:{}: offset overflow",
            file_location.file(),
            file_location.line(),
            ))?;
    let bytes = array
        .get(offset..end)
        .ok_or_else(|| anyhow::anyhow!("{}:{}: out of bounds read",
            file_location.file(),
            file_location.line(),
            ))?;
    Ok(unsafe {
        std::ptr::read_unaligned(bytes.as_ptr() as *const T)
    })
}

#[track_caller]
pub fn write_to_slice<T: Pod>(array: &mut [u8], offset: u64, value: impl TryInto<T>) -> Result<()> {
    let file_location = std::panic::Location::caller();
    let offset = offset as usize;
    let value: T = value.
        try_into()
        .map_err(|_| anyhow!("{}:{}: type conversion failed",
                file_location.file(),
                file_location.line(),
        ))?;
    let size = std::mem::size_of::<T>();
    if offset + size > array.len() {
        bail!("{}:{}: write out of bounds",
            file_location.file(),
            file_location.line(),
        )
    }
    let bytes = unsafe {
        std::slice::from_raw_parts(&value as *const T as *const u8, size)
    };
    array[offset..][..size].copy_from_slice(bytes);
    Ok(())
}

#[track_caller]
fn rel_i32(target: u64, source: u64) -> Result<i32> {
    let file_location = std::panic::Location::caller();
    let relative_offset = (target as i128) - (source as i128);
    relative_offset
        .try_into()
        .map_err(|_| anyhow!("{}:{}: relative offset outside i32 range",
                file_location.file(),
                file_location.line(),
        ))
}

#[track_caller]
pub fn write_rel_i32(asm: &mut Vec<u8>, location: u64, offset: u64, target: u64, bytes_to_next_instr: u64) -> Result<()> {
    write_to_slice::<i32>(asm, offset, rel_i32(target, location + offset + bytes_to_next_instr)?)
}

#[track_caller]
pub fn get_hook_bytes(code_location: u64, hook_location: u64, original_instruction_size: usize) -> Result<Vec<u8>> {
    let mut bytes = vec![0xE9, 0x00, 0x00, 0x00, 0x00];
    let nop_num = original_instruction_size.saturating_sub(5);
    let nops = vec![0x90; nop_num];
    bytes.extend_from_slice(&nops);
    write_rel_i32(&mut bytes, hook_location, 1, code_location, 4)?;
    Ok(bytes)
}