pub mod error;

use crate::{
    attached,
    sys::error::{ProcResult, ProcessError, PtraceAction, WriteType},
};
use libc::{NT_PRSTATUS, PTRACE_GETREGSET, PTRACE_SETREGSET};
use nix::{
    sys::{
        ptrace::{
            self,
            regset::{NT_PRFPREG, NT_PRSTATUS},
        },
        uio::{RemoteIoVec, process_vm_readv, process_vm_writev},
        wait::waitpid,
    },
    unistd::Pid,
};
use pelite::Pod;
use utils::{
    object::AsmFolder,
    slice_ops::{SliceError, write_to_slice},
};
use std::{
    any::type_name,
    env, fs,
    hint::spin_loop,
    io::{IoSlice, IoSliceMut},
    mem::zeroed,
    ptr, slice,
    sync::{LazyLock, Mutex},
    thread,
    time::{Duration, Instant},
};
use thiserror::Error;

static PTRACE_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

static ASM_LIB32_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sys32.bin"));
static ASM_LIB64_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sys64.bin"));
static ASM32: LazyLock<AsmFolder> = LazyLock::new(|| bincode::deserialize(ASM_LIB32_BYTES).unwrap());
static ASM64: LazyLock<AsmFolder> = LazyLock::new(|| bincode::deserialize(ASM_LIB64_BYTES).unwrap());

#[track_caller]
pub fn read_unsafe<T: Pod>(address: u64) -> ProcResult<T> {
    unsafe {
        let pid = attached::pid();
        let mut value = std::mem::zeroed::<T>();
        let size = std::mem::size_of::<T>();
        let local_iov = IoSliceMut::new(slice::from_raw_parts_mut(&mut value as *mut T as *mut u8, size));
        let remote_iov = RemoteIoVec { base: address as usize, len: size };

        let nread = match process_vm_readv(pid, &mut [local_iov], &[remote_iov]) {
            Ok(n) => n,
            Err(err) => return Err(ProcessError::io(
                error::AccessType::Read(type_name::<T>()),
                address,
                std::io::Error::from(err),
            )),
        };
        if nread != size {
            return Err(ProcessError::partial_access(
                error::AccessType::Read(type_name::<T>()),
                nread,
                address,
            ));
        }
        Ok(value)
    }
}

#[track_caller]
pub fn write_unsafe<T: Pod>(address: u64, value: T) -> ProcResult {
    unsafe {
        let pid = attached::pid();
        let size = std::mem::size_of::<T>();
        let local_iov = IoSlice::new(slice::from_raw_parts(&value as *const T as *const u8, size));
        let remote_iov = RemoteIoVec { base: address as usize, len: size };

        let nwritten = match process_vm_writev(pid, &[local_iov], &[remote_iov]) {
            Ok(n) => n,
            Err(err) => return Err(ProcessError::io(
                error::AccessType::Write(WriteType::Type(type_name::<T>())),
                address,
                std::io::Error::from(err),
            )),
        };
        if nwritten != size {
            return Err(ProcessError::partial_access(
                error::AccessType::Write(WriteType::Type(type_name::<T>())),
                nwritten,
                address,
            ));
        }
        Ok(())
    }
}

#[track_caller]
pub fn write_bytes_unsafe(address: u64, data: &[u8]) -> ProcResult {
    let pid = attached::pid();
    let size = data.len();
    let local_iov = IoSlice::new(data);
    let remote_iov = RemoteIoVec { base: address as usize , len: size };

    let nwritten = match process_vm_writev(pid, &[local_iov], &[remote_iov]) {
        Ok(n) => n,
        Err(err) => return Err(ProcessError::io(
            error::AccessType::Write(WriteType::Bytes(size)),
            address,
            std::io::Error::from(err),
        )),
    };
    if nwritten != size {
        return Err(ProcessError::partial_access(
            error::AccessType::Write(WriteType::Bytes(size)),
            nwritten,
            address,
        ));
    }
    Ok(())
}

pub fn spawn_thread_release(
    spawn_code_address: u64,
    thread_start_address: u64,
    thread_code: Vec<u8>,
    create_thread_pointer: u64,
    close_handle_pointer: u64,
) -> ProcResult {
    if attached::is_32() {
        write_bytes_unsafe(thread_start_address, &thread_code)?;
        run_win32_thread(
            spawn_code_address,
            thread_start_address,
            create_thread_pointer,
            close_handle_pointer,
        )
    } else {
        write_bytes_unsafe(thread_start_address, &thread_code)?;
        run_win64_thread(
            spawn_code_address,
            thread_start_address,
            create_thread_pointer,
            close_handle_pointer,
        )
    }
}

pub fn spawn_thread_join(
    spawn_code_address: u64,
    thread_start_address: u64,
    mut thread_code: Vec<u8>,
    create_thread_pointer: u64,
    close_handle_pointer: u64,
) -> ProcResult {
    let running_flag = thread_start_address.saturating_sub(1);
    write_unsafe::<u8>(running_flag, 0x1)?;

    if attached::is_32() {
        append_32bit_flag_setter(thread_start_address, &mut thread_code)?;
        write_bytes_unsafe(thread_start_address, &thread_code)?;
        run_win32_thread(
            spawn_code_address,
            thread_start_address,
            create_thread_pointer,
            close_handle_pointer,
        )?;
    } else {
        append_64bit_flag_setter(thread_start_address, &mut thread_code)?;
        write_bytes_unsafe(thread_start_address, &thread_code)?;
        run_win64_thread(
            spawn_code_address,
            thread_start_address,
            create_thread_pointer,
            close_handle_pointer,
        )?;
    }

    let start = Instant::now();
    let timeout = Duration::from_millis(50);
    loop {
        if read_unsafe::<u8>(running_flag)? == 0x0 {
            return Ok(())
        }
        if start.elapsed() > timeout {
            return Err(ProcessError::RemoteThreadReturn { timeout })
        }
        spin_loop();
    }
}

fn run_win64_thread(
    spawn_code_address: u64,
    thread_start_address: u64,
    create_thread_pointer: u64,
    close_handle_pointer: u64,
) -> ProcResult {
    let pid = attached::pid();
    let start = Instant::now();
    let timeout = Duration::from_millis(50);

    loop {
        if start.elapsed() > timeout {
            return Err(ProcessError::RemoteThreadCreate { os_error: Some(libc::ETIMEDOUT) });
        }

        let handle = PTRACE_MUTEX.lock().unwrap();
        ptrace::attach(pid).map_err(|e| ProcessError::ptrace(PtraceAction::Attach, e))?;
        waitpid(pid, None).map_err(|e| ProcessError::ptrace(PtraceAction::Wait, e))?;

        let start = attached::module_base();
        let original_regs = ptrace::getregset::<NT_PRSTATUS>(pid)
            .map_err(|e| ProcessError::ptrace(PtraceAction::GetRegs, e))?;

        if start < original_regs.rip && original_regs.rip < start + 0x5E03000 {
            let original_fp_regs = ptrace::getregset::<NT_PRFPREG>(pid)
                .map_err(|e| ProcessError::ptrace(PtraceAction::GetRegs, e))?;

            let mut regs = original_regs;

            regs.rip = spawn_code_address;
            regs.rsp = regs.rsp.strict_sub(0x100) & !0xFu64;

            let flag_loc = spawn_code_address.strict_sub(1);

            let fun = ASM64.get_function("run_thread");
            let mut asm = fun.get_bytes();

            write_to_slice::<u64>(&mut asm, fun.reloc("code_address"), thread_start_address)?;
            write_to_slice::<u64>(&mut asm, fun.reloc("create_thread"), create_thread_pointer)?;
            write_to_slice::<u64>(&mut asm, fun.reloc("close_handle"), close_handle_pointer)?;
            write_to_slice::<u64>(&mut asm, fun.reloc("flag_loc"), flag_loc)?;

            write_unsafe::<u8>(flag_loc, 0x0)?;
            write_bytes_unsafe(spawn_code_address, &asm)?;

            ptrace::setregset::<NT_PRSTATUS>(pid, regs)
                .map_err(|e| ProcessError::ptrace(PtraceAction::SetRegs, e))?;

            ptrace::cont(pid, None).map_err(|e| ProcessError::ptrace(PtraceAction::Cont, e))?;
            waitpid(pid, None).map_err(|e| ProcessError::ptrace(PtraceAction::Wait, e))?;

            ptrace::setregset::<NT_PRSTATUS>(pid, original_regs)
                .map_err(|e| ProcessError::ptrace(PtraceAction::SetRegs, e))?;
            ptrace::setregset::<NT_PRFPREG>(pid, original_fp_regs)
                .map_err(|e| ProcessError::ptrace(PtraceAction::SetRegs, e))?;
            ptrace::detach(pid, None)
                .map_err(|e| ProcessError::ptrace(PtraceAction::Detach, e))?;

            return check_success_flag(flag_loc);
        } else {
            ptrace::detach(pid, None)
                .map_err(|e| ProcessError::ptrace(PtraceAction::Detach, e))?;
            drop(handle);
            thread::sleep(Duration::from_micros(10));
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
struct i386Regs {
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,
    eax: u32,
    ds: u16,
    __ds: u16,
    es: u16,
    __es: u16,
    fs: u16,
    __fs: u16,
    gs: u16,
    __gs: u16,
    orig_eax: u32,
    eip: u32,
    cs: u16,
    __cs: u16,
    eflags: u32,
    esp: u32,
    ss: u16,
    __ss: u16,
}

fn run_win32_thread(
    spawn_code_address: u64,
    thread_start_address: u64,
    create_thread_pointer: u64,
    close_handle_pointer: u64,
) -> Result<(), ProcessError> {
    let pid = attached::pid();
    let start = Instant::now();
    let timeout = Duration::from_millis(50);

    loop {
        if start.elapsed() > timeout {
            return Err(ProcessError::RemoteThreadCreate { os_error: Some(libc::ETIMEDOUT) });
        }

        let handle = PTRACE_MUTEX.lock().unwrap();

        unsafe {
            ptrace::attach(pid).map_err(|e| ProcessError::ptrace(PtraceAction::Attach, e))?;
            waitpid(pid, None).map_err(|e| ProcessError::ptrace(PtraceAction::Wait, e))?;

            let mut regs_buf: [u8; size_of::<i386Regs>()] = zeroed();
            let mut iov = libc::iovec {
                iov_base: regs_buf.as_mut_ptr() as *mut libc::c_void,
                iov_len: regs_buf.len(),
            };

            libc::ptrace(
                PTRACE_GETREGSET,
                pid,
                NT_PRSTATUS as *mut libc::c_void,
                &mut iov as *mut _ as *mut libc::c_void,
            );

            let regs_ptr = regs_buf.as_mut_ptr() as *mut i386Regs;
            let original_regs = ptr::read_unaligned(regs_ptr);
            let eip = original_regs.eip as u64;
            let start = attached::module_base();

            if start < eip && eip < start + 0x5E03000 {
                let mut regs = original_regs.clone();

                let flag_loc = spawn_code_address.saturating_sub(1);

                let fun = ASM32.get_function("run_thread");
                let mut asm = fun.get_bytes();

                write_to_slice::<u32>(&mut asm, fun.reloc("code_address"), thread_start_address)?;
                write_to_slice::<u32>(&mut asm, fun.reloc("create_thread"), create_thread_pointer)?;
                write_to_slice::<u32>(&mut asm, fun.reloc("close_handle"), close_handle_pointer)?;
                write_to_slice::<u32>(&mut asm, fun.reloc("flag_loc"), flag_loc)?;

                write_unsafe::<u8>(flag_loc, 0x0)?;
                write_bytes_unsafe(spawn_code_address, &asm)?;

                regs.eip = spawn_code_address as u32;

                ptr::write_unaligned(regs_ptr, regs);

                libc::ptrace(
                    PTRACE_SETREGSET,
                    pid,
                    libc::NT_PRSTATUS as *mut libc::c_void,
                    &mut iov as *mut _ as *mut libc::c_void
                );

                ptrace::cont(pid, None).map_err(|e| ProcessError::ptrace(PtraceAction::Cont, e))?;
                waitpid(pid, None).map_err(|e| ProcessError::ptrace(PtraceAction::Wait, e))?;

                ptr::write_unaligned(regs_ptr, original_regs);

                libc::ptrace(PTRACE_SETREGSET, pid, 1, &mut iov as *mut _ as *mut libc::c_void);
                ptrace::detach(pid, None)
                    .map_err(|e| ProcessError::ptrace(PtraceAction::Detach, e))?;

                return check_success_flag(flag_loc);
            } else {
                ptrace::detach(pid, None)
                    .map_err(|e| ProcessError::ptrace(PtraceAction::Detach, e))?;
                drop(handle);
                thread::sleep(Duration::from_micros(10));
            }
        }
    }
}

fn check_success_flag(flag_loc: u64) -> ProcResult {
    let flag = read_unsafe::<u8>(flag_loc)?;
    if flag != 0x0 {
        Ok(())
    } else {
        // check CreateThread error code todo
        Err(ProcessError::RemoteThreadCreate { os_error: Some(0) })
    }
}

const FLAG_SETTER_64: [u8; 14] = [
    0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00,   // movabs rax, flag_loc
    0x00, 0x00, 0x00,
    0xC6, 0x00, 0x00,                           // mov BYTE PTR [rax], 0x0
    0xC3,                                       // ret
];

const FLAG_SETTER_32: [u8; 9] = [
    0xB8, 0x00, 0x00, 0x00, 0x00,               // mov eax, flag_loc
    0xC6, 0x00, 0x00,                           // mov BYTE PTR [eax], 0x0
    0xC3,                                       // ret
];

fn append_64bit_flag_setter(location: u64, asm_head: &mut Vec<u8>) -> Result<(), SliceError> {
    let mut asm_tail = FLAG_SETTER_64;
    write_to_slice::<u64>(&mut asm_tail, 2, location.saturating_sub(1))?;
    asm_head.pop();
    asm_head.extend_from_slice(&asm_tail);
    Ok(())
}

fn append_32bit_flag_setter(location: u64, asm_head: &mut Vec<u8>) -> Result<(), SliceError> {
    let mut asm_tail = FLAG_SETTER_32;
    write_to_slice::<u32>(&mut asm_tail, 1, location.saturating_sub(1))?;
    asm_head.pop();
    asm_head.extend_from_slice(&asm_tail);
    Ok(())
}

#[derive(Debug, Error)]
pub enum UptimeError {
    #[error("Could not read file: {error}")]
    File { error: std::io::Error },
    #[error("{error}")]
    Float { error: std::num::ParseFloatError },
    #[error("Could not determine process start time")]
    StartTime,
    #[error("Could not read system uptime")]
    UpTime,
}

pub fn get_process_uptime(pid: Pid) -> Result<f64, UptimeError> {
    let stat = fs::read_to_string(format!("/proc/{}/stat", pid))
        .map_err(|e| UptimeError::File { error: e })?;
    let start_ticks: f64 = stat
        .split_whitespace()
        .nth(21)
        .ok_or(UptimeError::StartTime)?
        .parse()
        .map_err(|e| UptimeError::Float { error: e })?;

    let system_uptime_str = fs::read_to_string("/proc/uptime")
        .map_err(|e| UptimeError::File { error: e })?;
    let system_uptime: f64 = system_uptime_str
        .split_whitespace()
        .next()
        .ok_or(UptimeError::UpTime)?
        .parse()
        .map_err(|e| UptimeError::Float { error: e })?;

    let system_tick_rate = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as f64;

    let process_start = start_ticks / system_tick_rate;
    Ok(system_uptime - process_start)
}