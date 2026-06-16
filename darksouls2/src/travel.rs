use crate::{
    mem::*,
    offsets::{
        ChainReadExt,
        code_cave::CaveOffset,
        functions,
        game_manager_imp::{self, event_manager_offsets},
    },
    resources::{bonfires::Bonfire, bosses::Boss, scholar, vanilla},
    utils::character_loaded_check,
};
use gubtool_core::sys::error::ProcResult;
use utils::slice_ops::*;

const DEFAULT_TRANSITION_MODE: u32 = 6;
const DEFAULT_SPAWN_ANIM: u32 = 3;

#[repr(C, packed)]
struct WarpRequest {
    kind: u32,
    transition_mode: u32,
    map_id: u32,
    unk_0c: i32,
    post_warp_demo_id: u32,
    spawn_anim: u32,
    payload: Payload,
    quaternion: [f32; 4],
    pre_warp_demo_id: u32,
    post_submit_flag: u8,
    post_submit_special_flag: u8,
    _pad: u16,
}

#[repr(C, packed)]
union Payload {
    pos: [f32; 4],
    payload_id: u32,
}

enum WarpKind {
    Direct = 0,
    DirectWithOffset = 1,
    MapOnly = 2,
    Bonfire = 3,
    EventPoint = 4
}

impl Default for WarpRequest {
    fn default() -> Self {
        Self {
            kind: 0,
            transition_mode: DEFAULT_TRANSITION_MODE,
            map_id: 0,
            unk_0c: -1,
            post_warp_demo_id: 0,
            spawn_anim: DEFAULT_SPAWN_ANIM,
            payload: Payload { payload_id: 0 },
            quaternion: [0.0; 4],
            pre_warp_demo_id: 0,
            post_submit_flag: 0,
            post_submit_special_flag: 0,
            _pad: 0,
        }
    }
}

impl WarpRequest {
    fn to_array(&self) -> [u8; std::mem::size_of::<Self>()] {
        unsafe { std::mem::transmute_copy(self) }
    }
}

impl Boss {
    pub fn warp(&self) -> anyhow::Result<()> {
        character_loaded_check()?;

        let request = WarpRequest {
            kind: WarpKind::Direct as u32,
            payload: Payload { pos: self.pos },
            quaternion: self.quaternion,
            map_id: self.map_id as u32,
            ..Default::default()
        };
        warp(request)?;
        Ok(())
    }
}

impl Bonfire {
    pub fn warp(&self) -> anyhow::Result<()> {
        character_loaded_check()?;

        let request = WarpRequest {
            kind: WarpKind::Bonfire as u32,
            map_id: self.map_id as u32,
            payload: Payload { payload_id: self.bonfire_id },
            ..Default::default()
        };
        warp(request)?;
        Ok(())
    }
}

fn warp(request: WarpRequest) -> ProcResult {
    // let _handle = TRAVEL_MUTEX.try_lock()
        // .map_err(|_| anyhow!("Is already travelling"))?;

    let request_loc = CaveOffset::WarpRequestStruct.addr();
    let location = CaveOffset::WarpRequestAsm.addr();

    write_bytes(request_loc, &request.to_array())?;

    let warp_manager = read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::EVENT_MANAGER)
        .read_offset(event_manager_offsets::EVENT_WARP_MANAGER)?;

    let asm = if is_scholar() {
        let fun = scholar::ASM.get_function("warp");
        let mut asm = fun.get_bytes();
        write_to_slice::<u64>(&mut asm, fun.reloc("warp_manager"), warp_manager)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("request_loc"), request_loc)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("fn_request_warp"), functions::warp())?;
        asm
    } else {
        let fun = vanilla::ASM.get_function("warp");
        let mut asm = fun.get_bytes();
        write_to_slice::<u32>(&mut asm, fun.reloc("warp_manager"), warp_manager)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("request_loc"), request_loc)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("fn_request_warp"), functions::warp())?;
        asm
    };
    spawn_thread_join(location, asm)
}