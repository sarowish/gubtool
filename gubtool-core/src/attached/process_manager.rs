use crate::{
    attached::{
        self, AttachError, GameProcess, ParseState,
        parse::{VALID_COMMS, parse_process},
        process_exists,
    },
    game_version::Game,
    sys::Pid,
};

pub struct ProcessManager {
    processes: Vec<GameProcess>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    pub fn refresh(&mut self) {
        for (pid, name) in Self::processes() {
            for (valid_comm, game) in VALID_COMMS {
                if name == *valid_comm
                    && self.processes.iter().all(|p| pid != p.pid.as_u32())
                {
                    self.processes.push(parse_process(game, Pid::new(pid), name.clone()));
                }
            }
        }

        #[cfg(unix)]
        self.processes.retain(|p| {
            p.exists()
        });

        #[cfg(windows)]
        self.processes.retain(|p| {
            if !p.exists() {
                unsafe {
                    let _ = windows::Win32::Foundation::CloseHandle(p.handle);
                }
                false
            } else {
                true
            }
        });
    }

    pub fn try_auto_attach(&mut self) -> Option<Result<(), AttachError>> {
        self.refresh();

        for process in &self.processes {
            match process.parse_state {
                ParseState::Valid => {
                    return Some(process.attach());
                }
                ParseState::Invalid(_)  => continue
            }
        }
        None
    }

    pub fn detach_if_invalid(&mut self) -> Option<Game> {
        if let Some(exists) = process_exists() {
            if !exists {
                let game = attached::game();
                attached::detach();
                return game;
            }
        }
        None
    }

    pub fn valid_processes(&self) -> &Vec<GameProcess> {
        &self.processes
    }

    #[cfg(unix)]
    fn processes() -> impl Iterator<Item = (u32, String)> {
        use std::fs;

        std::fs::read_dir("/proc")
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_name = entry.file_name();
                let pid_str = file_name.to_string_lossy();

                let pid: u32 = pid_str.parse().ok()?;

                let cmdline_path = format!("/proc/{}/comm", pid);
                let name = fs::read_to_string(cmdline_path).ok()?;

                Some((pid, name.trim().to_string()))
            })
    }

    #[cfg(windows)]
    pub fn processes() -> impl Iterator<Item = (u32, String)> {
        let mut out = Vec::new();

        unsafe {
            use windows::Win32::System::Diagnostics::ToolHelp::Process32NextW;
            use windows::Win32::{
                Foundation::CloseHandle,
                System::Diagnostics::ToolHelp::{
                    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, TH32CS_SNAPPROCESS,
                },
            };

            let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
                .expect("failed to create process snapshot");

            let mut entry = PROCESSENTRY32W::default();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

            if Process32FirstW(handle, &mut entry).is_ok() {
                loop {
                    let len = entry
                        .szExeFile
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(entry.szExeFile.len());

                    let name = String::from_utf16_lossy(&entry.szExeFile[..len]);

                    out.push((entry.th32ProcessID, name));

                    if Process32NextW(handle, &mut entry).is_err() {
                        break;
                    }
                }
            }

            let _ = CloseHandle(handle);
        }

        out.into_iter()
    }
}