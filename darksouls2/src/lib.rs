pub mod attach;
pub mod bonfire;
pub mod chr_ctrl;
pub mod covenant;
pub mod event;
pub mod game_state;
pub mod item;
mod mem;
pub mod menu;
mod pointer_cache;
mod offsets;
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
    let _ = target::act_logger().update();
    target::target().update();
    player::STATS.write().unwrap().read();
}

pub fn is_player_loaded() -> bool {
    GAME_STATE.loaded.load(Ordering::Relaxed)
}