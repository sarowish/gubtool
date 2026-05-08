pub mod items;
pub mod warps;

pub(super) mod scholar {
    use crate::core::object::AsmFolder;
    use std::sync::LazyLock;

    static ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/scholar.bin"));
    pub static ASM: LazyLock<AsmFolder> =
        LazyLock::new(|| bincode::deserialize(ASM_LIB_BYTES).unwrap());
}

pub(super) mod vanilla {
    use crate::core::object::AsmFolder;
    use std::sync::LazyLock;

    static ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vanilla.bin"));
    pub static ASM: LazyLock<AsmFolder> =
        LazyLock::new(|| bincode::deserialize(ASM_LIB_BYTES).unwrap());
}
