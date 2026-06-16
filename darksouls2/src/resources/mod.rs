pub mod map_ids;
pub mod bonfires;
pub mod bosses;
pub mod covenants;
pub mod event_flags;
pub mod items;
pub mod menus;
pub mod scholar_patterns;
pub mod vanilla_patterns;
pub mod versions_module_offsets;

pub(super) mod scholar {
    use utils::object::AsmFolder;
    use std::sync::LazyLock;

    static ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/scholar.bin"));
    pub static ASM: LazyLock<AsmFolder> =
        LazyLock::new(|| bincode::deserialize(ASM_LIB_BYTES).unwrap());
}

pub(super) mod vanilla {
    use utils::object::AsmFolder;
    use std::sync::LazyLock;

    static ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vanilla.bin"));
    pub static ASM: LazyLock<AsmFolder> =
        LazyLock::new(|| bincode::deserialize(ASM_LIB_BYTES).unwrap());
}
