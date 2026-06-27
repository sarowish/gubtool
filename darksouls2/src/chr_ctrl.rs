use crate::{
    offsets::{self, ChainReadExt, chr_ctrl::stats_offsets},
    resources::covenants::Covenant,
};
use gubtool_core::sys::error::ProcResult;

pub type ChrCtrl = ProcResult<u64>;

pub trait ChrCtrlExt {
    fn is_valid_chr(self) -> ProcResult<bool>;
    fn get_hp(&self) -> ProcResult<i32>;
    fn set_hp(&self, val: i32) -> ProcResult;
    fn get_min_hp(&self) -> ProcResult<i32>;
    fn set_min_hp(&self, val: i32) -> ProcResult;
    fn max_hp(&self) -> ProcResult<i32>;
    fn is_no_death(&self) -> bool;
    fn set_no_death(&self, state: bool) -> ProcResult;
    fn coords(&self) -> ProcResult<[f32; 3]>;
    fn poise(&self) -> ProcResult<f32>;
    fn max_poise(&self) -> ProcResult<f32>;
    fn posture(&self) -> ProcResult<f32>;
    fn max_posture(&self) -> ProcResult<f32>;

    fn rot_quaternion(&self) -> ProcResult<[f32; 4]>;
    fn get_covenant(&self) -> ProcResult<Covenant>;
    fn set_covenant(&self, covenant: Covenant) -> ProcResult;

    fn chr_id(&self) -> ProcResult<i32>;

    fn param_pointer(&self) -> ProcResult<u64>;
    fn stats_pointer(&self) -> ProcResult<u64>;

    fn name_from_chr_id(&self) -> &'static str;
}

impl ChrCtrlExt for ChrCtrl {
    fn is_valid_chr(self) -> ProcResult<bool> {
        if self? == 0x0 {
            return Ok(false)
        }
        let health = self.get_hp()?;
        let max_health = self.max_hp()?;
        Ok(health >= 0
            && max_health > 0
            && health < 10000000
            && max_health < 10000000
            && (health as f32) < (max_health as f32) * 1.5)
    }

    fn get_hp(&self) -> ProcResult<i32> {
        self.add_offset(offsets::chr_ctrl::HEALTH)
            .read::<i32>()
    }

    fn set_hp(&self, val: i32) -> ProcResult {
        let max = self.max_hp()?;
        self.add_offset(offsets::chr_ctrl::HEALTH)
            .write::<i32>(val.min(max))
    }

    fn get_min_hp(&self) -> ProcResult<i32> {
        self.add_offset(offsets::chr_ctrl::MIN_HEALTH)
            .read::<i32>()
    }

    fn set_min_hp(&self, val: i32) -> ProcResult {
        self.add_offset(offsets::chr_ctrl::MIN_HEALTH)
            .write::<i32>(val)
    }

    fn max_hp(&self) -> ProcResult<i32> {
        self.add_offset(offsets::chr_ctrl::MAX_HEALTH)
            .read::<i32>()
    }

    fn is_no_death(&self) -> bool {
        self.get_min_hp()
            .map(|val| val == 1)
            .unwrap_or_default()
    }

    fn set_no_death(&self, state: bool) -> ProcResult {
        let val = if state { 1 } else { -99999 };
        self.set_min_hp(val)
    }

    fn coords(&self) -> ProcResult<[f32; 3]> {
        self.add_offset(offsets::chr_ctrl::COORDS)
            .read::<[f32; 3]>()
    }

    fn chr_id(&self) -> ProcResult<i32> {
        self.param_pointer()
            .add_offset(offsets::chr_ctrl::CHR_ID)
            .read::<i32>()
    }

    fn poise(&self) -> ProcResult<f32> {
        self.add_offset(offsets::chr_ctrl::POISE)
            .read::<f32>()
    }

    fn max_poise(&self) -> ProcResult<f32> {
        self.add_offset(offsets::chr_ctrl::MAX_POISE)
            .read::<f32>()
    }

    fn posture(&self) -> ProcResult<f32> {
        self.add_offset(offsets::chr_ctrl::POSTURE)
            .read::<f32>()
    }

    fn max_posture(&self) -> ProcResult<f32> {
        self.add_offset(offsets::chr_ctrl::MAX_POSTURE)
            .read::<f32>()
    }

    fn rot_quaternion(&self) -> ProcResult<[f32; 4]> {
        let [m00, m01, m02, _, m10, m11, m12, _, m20, m21, m22, _] =
            self.add_offset(offsets::chr_ctrl::ROTATION)
            .read::<[f32; 12]>()?;

        let matrix = glam::Mat3::from_cols(
            glam::Vec3::new(m00, m01, m02),
            glam::Vec3::new(m10, m11, m12),
            glam::Vec3::new(m20, m21, m22),
        );
        let q = glam::Quat::from_mat3(&matrix);

        Ok([q.x, q.y, q.z, q.w])
    }

    fn get_covenant(&self) -> ProcResult<Covenant> {
        self.stats_pointer()
            .add_offset(stats_offsets::COVENANT)
            .read::<u8>()
            .map(|val| Covenant::try_from(val).unwrap_or(Covenant::None))
    }

    fn set_covenant(&self, covenant: Covenant) -> ProcResult {
        self.stats_pointer()
            .add_offset(stats_offsets::COVENANT)
            .write::<u8>(covenant as u8)
    }

    fn param_pointer(&self) -> ProcResult<u64> {
        self.read_offset(offsets::chr_ctrl::PARAMS_PTR)
    }

    fn stats_pointer(&self) -> ProcResult<u64> {
        self.read_offset(offsets::chr_ctrl::STATS_PTR)
    }

    fn name_from_chr_id(&self) -> &'static str {
        crate::resources::chr_names::CHR_NAMES
            .get(&self.chr_id().unwrap_or_default())
            .map_or("", |v| *v)
    }
}