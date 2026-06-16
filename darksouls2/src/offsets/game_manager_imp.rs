use crate::{
    mem::is_scholar,
    offsets::{Offset, module_offsets::module_offsets},
};

pub fn base_ptr() -> u64 {
    gubtool_core::attached::module_base() + module_offsets().base_ptrs.game_manager_imp
}

pub const CHARACTER_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x18,
};

pub const CAMERA_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x20,
};

pub const AI_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x28,
};

pub const APP_RESOURCE_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x30,
};

pub const ENEMY_GENERATOR_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x40,
};

pub const TARGET_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x48,
};

pub const PAD_OWNERSHIP_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x50,
};

pub const EVENT_MANAGER: Offset = Offset {
    vanilla: 0x44,
    scholar: 0x70,
};

pub const FACE_GEN_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x80,
};

pub const RUMBLE_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x88,
};

pub const SIGN_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x90,
};

pub const STATE_ACT_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0xA0,
};

pub const GAME_DATA_MANAGER: Offset = Offset {
    vanilla: 0x60,
    scholar: 0xA8,
};

pub const SAVE_LOAD_SYSTEM: Offset = Offset {
    vanilla: 0,
    scholar: 0xB8,
};

pub const APP_DLC_CONTENTS_INFO_ACCESSOR: Offset = Offset {
    vanilla: 0,
    scholar: 0xC8,
};

pub const PLAYER_CTRL: Offset = Offset {
    vanilla: 0x74,
    scholar: 0xD0,
};

pub const LOADING_FLAG: Offset = Offset {
    vanilla: 0xDFC,
    scholar: 0x24BC,
};

pub mod event_manager_offsets {
    use crate::offsets::Offset;

    pub const EVENT_FLAG_MANAGER: Offset = Offset {
        vanilla: 0x10,
        scholar: 0x20,
    };

    pub const EVENT_WARP_MANAGER: Offset = Offset {
        vanilla: 0x38,
        scholar: 0x70,
    };

    pub const EVENT_BONFIRE_MANAGER: Offset = Offset {
        vanilla: 0x2C,
        scholar: 0x58,
    };

    pub const RESPAWN_MAP: Offset = Offset {
        vanilla: 0xB4,
        scholar: 0x164 ,
    };

    pub const RESPAWN_BONFIRE: Offset = Offset {
        vanilla: 0xBC,
        scholar: 0x16C ,
    };

    pub const EVENT_WINDOW_MANAGER: Offset = Offset {
        vanilla: 0x28,
        scholar: 0x50,
    };
    pub mod bonfire_manager_offsets {
        use crate::offsets::Offset;

        pub const ARRAY_BASE: Offset = Offset {
            vanilla: 0x10,
            scholar: 0x20,
        };

        pub const COUNT: Offset = Offset {
            vanilla: 0x14,
            scholar: 0x28,
        };
    }
}

pub const QUITOUT: Offset = Offset {
    vanilla: 0xDF1,
    scholar: 0x24B1,
};

pub const PX_WORLD: Offset = Offset {
    vanilla: 0x280,
    scholar: 0x660,
};

pub fn player_coords_chain() -> [u64; 7] {
    match crate::mem::is_scholar() {
        true => [base_ptr(), PX_WORLD.resolve(), 0x18, 0x1F8, 0x18, 0x8, 0x1A0],
        false => [base_ptr(), PX_WORLD.resolve(), 0xC, 0x168, 0xC, 0x4, 0x120],
    }
}

pub const DL_BACK_ALLOCATOR: Offset = Offset {
    vanilla: 0xCC4,
    scholar: 0x22E0,
};

pub mod dl_back_allocator_offsets {
    use crate::offsets::Offset;

    pub const UNK_FLAG: Offset = Offset {
        vanilla: 0x1A3,
        scholar: 0x30F,
    };

    pub const REF_COUNT: Offset = Offset {
        vanilla: 0x1B0,
        scholar: 0x31C,
    };
}

pub fn fe_item_select_menu_chain() -> [u64; 7] {
    match is_scholar() {
        true => [base_ptr(), DL_BACK_ALLOCATOR.resolve(), 0x110, 0x10, 0x38, 0x30, 0x30],
        false => [base_ptr(), DL_BACK_ALLOCATOR.resolve(), 0x88, 0x8, 0x1C, 0x18, 0x18],
    }
}

pub mod fe_item_select_menu_offsets {
    use crate::offsets::Offset;

    pub const OPEN_FLAG: Offset = Offset {
        vanilla: 0x12,
        scholar: 0x1E,
    };
}

pub mod player_ctrl_offsets {
    use crate::offsets::Offset;

    pub const PLAYER_OPERATOR: Offset = Offset {
        vanilla: 0xAC,
        scholar: 0xE8,
    };
}

pub mod game_data_manager_offsets {
    use crate::offsets::Offset;

    pub const CLEARCOUNT_PTR: Offset = Offset {
        vanilla: 0x60,
        scholar: 0xC0,
    };

    pub mod clearcount_ptr_offsets {
        use crate::offsets::Offset;

        pub const CLEARCOUNT: Offset = Offset {
            vanilla: 0x68,
            scholar: 0x68,
        };
    }
}
