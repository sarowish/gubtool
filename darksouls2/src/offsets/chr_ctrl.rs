use crate::offsets::Offset;

pub const CHR_ID: Offset = Offset {
    vanilla: 0x0,
    scholar: 0x0,
};

pub const ROTATION: Offset = Offset {
    vanilla: 0x40,
    scholar: 0x60,
};

pub const ORIENTATION: Offset = Offset {
    vanilla: 0x60,
    scholar: 0x80,
};

pub const STATS_PTR: Offset = Offset {
    vanilla: 0x378,
    scholar: 0x490,
};

pub const PARAMS_PTR: Offset = Offset {
    vanilla: 0x20,
    scholar: 0x38,
};

pub const COORDS: Offset = Offset {
    vanilla: 0x80,
    scholar: 0x90,
};

pub const HEALTH: Offset = Offset {
    vanilla: 0xFC,
    scholar: 0x168,
};

pub const MIN_HEALTH: Offset = Offset {
    vanilla: 0x100,
    scholar: 0x16C,
};

pub const MAX_HEALTH: Offset = Offset {
    vanilla: 0x104,
    scholar: 0x170,
};

pub const POISE: Offset = Offset {
    vanilla: 0x1AC,
    scholar: 0x218,
};

pub const MIN_POISE: Offset = Offset {
    vanilla: 0x1B0,
    scholar: 0x21C,
};

pub const MAX_POISE: Offset = Offset {
    vanilla: 0x1B4,
    scholar: 0x220,
};

pub const POSTURE: Offset = Offset {
    vanilla: 0x14C,
    scholar: 0x1B8,
};

pub const MIN_POSTURE: Offset = Offset {
    vanilla: 0x150,
    scholar: 0x1BC,
};

pub const MAX_POSTURE: Offset = Offset {
    vanilla: 0x154,
    scholar: 0x1C0,
};

pub mod stats_offsets {
    use crate::offsets::Offset;

    pub const COVENANT: Offset = Offset {
        vanilla: 0x1A9,
        scholar: 0x1AD,
    };
}
