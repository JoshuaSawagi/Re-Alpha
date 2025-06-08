use smash::app::{
    self,
    *,
    lua_bind::*,
    FighterKineticEnergyMotion,
    FighterKineticEnergyController,
};
use smash::lua2cpp::*;
use smash::lib::{
    *,
    lua_const::*
};
use smash::phx::*;
use std::convert::TryInto;
use crate::consts::globals::CURRENT_FRAME;
use smash::app::BattleObjectModuleAccessor;
use crate::offsets;
use crate::utils::get_fighter_common_from_accessor;

pub trait Vec2Ext {
    fn new(x: f32, y: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
    fn mag(&self) -> f32;
    fn normalize(&self) -> Self;
}


impl Vec2Ext for Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Vec3Ext for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }
}

pub trait BomaExt {
    // INSTANCE
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
    // STATE
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;    
    unsafe fn status_frame(&mut self) -> i32;

}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }
    
    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }

    unsafe fn is_status(&mut self, kind: i32) -> bool {
        return StatusModule::status_kind(self) == kind;
    }
    
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }

    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        return StatusModule::situation_kind(self) == kind;
    }

    unsafe fn status_frame(&mut self) -> i32 {
        return get_fighter_common_from_accessor(self).global_table[CURRENT_FRAME]
            .get_i32();
    }
}
