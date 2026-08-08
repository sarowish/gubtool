pub mod attach;
pub mod chr_ins;
pub mod emevd;
pub mod event;
pub mod game_state;
pub mod item;
mod mem;
mod offsets;
mod phase_transition;
mod pointer_cache;
pub mod player;
pub mod resources;
pub mod target;
pub mod travel;
pub mod utility;
pub mod utils;

pub use attach::attach;
pub use pointer_cache::get_pointers;

use crate::{
    game_state::{GAME_STATE, STATE_FLAGS},
    pointer_cache::POINTER_CACHE,
};
use std::sync::atomic::Ordering;

pub fn init() {
    GAME_STATE.init();
}

pub fn reset() {
    POINTER_CACHE.reset_pointers();
    STATE_FLAGS.reset();
}

pub fn update() {
    STATE_FLAGS.update();
    GAME_STATE.update();
    target::target().update();
    player::player_game_data().read();
}

pub fn is_player_loaded() -> bool {
    GAME_STATE.loaded.load(Ordering::Relaxed)
}

pub fn is_dlc_available() -> bool {
    GAME_STATE.dlc.load(Ordering::Relaxed)
}