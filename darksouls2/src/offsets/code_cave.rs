pub const SIZE: usize = 0x5000;

fn base() -> u64 {
    engine::attached::module_base() + match crate::mem::is_scholar() {
        true => 0x1700000,
        false => 0x1250000,
    }
}

#[repr(u64)]
pub enum CaveOffset {
    ItemArgs = 0x0,                                     // 0x23
    ItemSpawnStack = 0x30,                              // 0x300
    SavedTargetPointer = 0x340,                         // u64
    WarpRequestStruct = 0x350,                          // 0x40
    CreditsModifyOnceFlag = 0x3A0,                      // u8

    StateHandlerFlags = 0xF00,                          // 0x100
    // Hooks
    PlayerNoDamageHook = 0x1000,                        // 0x2C
    InfinitePoiseHook = 0x1030,                         // 0x2C
    SaveTargetHook = 0x1060,                            // 0x13
    CreditsSkipHook = 0x1080,                           // 0x26
    FasterMenuHook = 0x10B0,                            // 0x1A
    EventLogHook = 0x10D0,                              // 0x39
    IvorySkipHook = 0x11B0,                             // 0xC1
    IvoryKnightsHook = 0x1290,                          // 0x24
    // Shellcode
    RunThreadAsm = 0x2001,                              // 0x60
    // Keep at least 16 bytes of buffer
    // for completion flag and appended flag setter
    WarpRequestAsm = 0x2070,                            // 0x29
    ItemSpawnAsm = 0x20B0,                              // 0x125
    SetEventAsm = 0x21F0,                               // 0x2F
    GiveSoulsAsm = 0x2230,                              // 0x29


    EventLogWriteIdx = 0x3FFC,                          // i32
    EventLogBuffer = 0x4000,                            // 0x1000
}

impl CaveOffset {
    pub fn addr(self) -> u64 {
        base() + self as u64
    }
}

pub mod item_args_offsets {
    pub const SHOULD_EXIT_FLAG: u64 = 0x0;              // u8
    pub const SHOULD_PROCESS_FLAG: u64 = 0x1;           // u8
    pub const ADJUST_QUANTITY_FLAG: u64 = 0x2;          // u8
    pub const MAX_QUANTITY: u64 = 0x3;                  // i32
    pub const ITEM_COUNT: u64 = 0x7;                    // i32
    pub const CURRENT_QUANTITY: u64 = 0xB;              // i32
    pub const STACK_COUNT: u64 = 0xF;                   // i32
    pub const ITEM_STRUCT: u64 = 0x13;                  // 0x16
}
pub mod item_struct_offsets {
    pub const ITEM_ID: u64 = 0x4;                       // i32
    pub const DURABILITY: u64 = 0x8;                    // f32
    pub const QUANTITY: u64 = 0xC;                      // i16
    pub const UPGRADE: u64 = 0xE;                       // u8
    pub const INFUSION: u64 = 0xF;                      // u8
}