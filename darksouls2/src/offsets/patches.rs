use engine::{Version, module_handle, version};

pub fn infinite_stamina() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x359B4F,
        Version::Vanilla1_0_12 => 0x35FCEF,
        Version::Scholar1_0_2 => 0x32D2AA,
        Version::Scholar1_0_3 => 0x33363A,
        _ => 0x0,
    }
}

pub fn infinite_consumables() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x231102,
        Version::Vanilla1_0_12 => 0x233F22,
        Version::Scholar1_0_2 => 0x1ABB5D,
        Version::Scholar1_0_3 => 0x1AF2CD,
        _ => 0x0,
    }
}

pub fn infinite_durability() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x36FFDA,
        Version::Vanilla1_0_12 => 0x37651A,
        Version::Scholar1_0_2 => 0x34516D,
        Version::Scholar1_0_3 => 0x34B67D,
        _ => 0x0,
    }
}

pub fn infinite_casts() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x22BFF9,
        Version::Vanilla1_0_12 => 0x22EE19,
        Version::Scholar1_0_2 => 0x1AB790,
        Version::Scholar1_0_3 => 0x1AEF00,
        _ => 0x0,
    }
}

pub fn no_soul_gain() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x273C94,
        Version::Vanilla1_0_12 => 0x276F74,
        Version::Scholar1_0_2 => 0x1FE9AA,
        Version::Scholar1_0_3 => 0x20249A,
        _ => 0x0,
    }
}

pub fn no_hollowing() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x3ACE17,
        Version::Vanilla1_0_12 => 0x3B37B7,
        Version::Scholar1_0_2 => 0x385199,
        Version::Scholar1_0_3 => 0x38BAF9,
        _ => 0x0,
    }
}

pub fn no_soul_loss() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x2C12C1,
        Version::Vanilla1_0_12 => 0x2C4D91,
        Version::Scholar1_0_2 => 0x266BF3,
        Version::Scholar1_0_3 => 0x26AFD3,
        _ => 0x0,
    }
}

pub fn player_hidden() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x44245E,
        Version::Vanilla1_0_12 => 0x44969E,
        Version::Scholar1_0_2 => 0x434DAA,
        Version::Scholar1_0_3 => 0x43BF2A,
        _ => 0x0,
    }
}

pub fn player_silent() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x1A1731,
        Version::Vanilla1_0_12 => 0x1A1970,
        Version::Scholar1_0_2 => 0x10E232,
        Version::Scholar1_0_3 => 0x10E306,
        _ => 0x0,
    }
}

pub fn menu_transition() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x187C9E,
        Version::Vanilla1_0_12 => 0x187E9E,
        Version::Scholar1_0_2 => 0xEF554,
        Version::Scholar1_0_3 => 0xEF614,
        _ => 0x0,
    }
}