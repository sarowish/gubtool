use crate::mem::is_scholar;

pub fn chr_id() -> u64 {
    0x0
}

pub fn rotation() -> u64 {
    match is_scholar() {
        true => 0x60,
        false => 0x40,
    }
}

pub fn orientation() -> u64 {
    match is_scholar() {
        true => 0x80,
        false => 0x60,
    }
}

pub fn stats_ptr() -> u64 {
    match is_scholar() {
        true => 0x490,
        false => 0x378,
    }
}

pub fn params_ptr() -> u64 {
    match is_scholar() {
        true => 0x38,
        false => 0x20,
    }
}

pub fn coords() -> u64 {
    match is_scholar() {
        true => 0x90,
        false => 0x80,
    }
}

pub fn health() -> u64 {
    match is_scholar() {
        true => 0x168,
        false => 0xFC,
    }
}

pub fn min_health() -> u64 {
    match is_scholar() {
        true => 0x16C,
        false => 0x100,
    }
}

pub fn max_health() -> u64 {
    match is_scholar() {
        true => 0x170,
        false => 0x104,
    }
}

pub fn poise() -> u64 {
    match is_scholar() {
        true => 0x218,
        false => 0x1AC,
    }
}

pub fn min_poise() -> u64 {
    match is_scholar() {
        true => 0x21C,
        false => 0x1B0,
    }
}

pub fn max_poise() -> u64 {
    match is_scholar() {
        true => 0x220,
        false => 0x1B4,
    }
}

pub fn posture() -> u64 {
    match is_scholar() {
        true => 0x1B8,
        false => 0x14C,
    }
}

pub fn min_posture() -> u64 {
    match is_scholar() {
        true => 0x1BC,
        false => 0x150,
    }
}

pub fn max_posture() -> u64 {
    match is_scholar() {
        true => 0x1C0,
        false => 0x154,
    }
}