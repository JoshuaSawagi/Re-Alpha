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
use crate::utils::compare_mask;

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
    unsafe fn get_jump_count(&mut self) -> i32;
    unsafe fn get_jump_count_max(&mut self) -> i32;
    unsafe fn is_cat_flag(&mut self, category: i32, fighter_pad_cmd_flag: i32) -> bool;
    unsafe fn is_prev_status(&mut self, kind: i32) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
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

    unsafe fn get_jump_count(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    unsafe fn get_jump_count_max(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    }
    
    unsafe fn is_cat_flag(&mut self, category: i32, fighter_pad_cmd_flag: i32) -> bool {
        let flag_mask = ControlModule::get_command_flag_cat(self, category);
        return compare_mask(flag_mask, fighter_pad_cmd_flag);
    }

    unsafe fn is_prev_status(&mut self, kind: i32) -> bool {
        return StatusModule::prev_status_kind(self, 0) == kind;
    }

    unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }
}

pub trait GetObjects {
    unsafe fn get_object(agent: &mut Self) -> &'static mut BattleObject;
    unsafe fn get_boma(agent: &mut Self) -> &'static mut BattleObjectModuleAccessor;
    
    unsafe fn object(&mut self) -> &'static mut BattleObject {
        Self::get_object(self)
    }

    unsafe fn boma(&mut self) -> &'static mut BattleObjectModuleAccessor {
        Self::get_boma(self)
    }
}

impl GetObjects for L2CAgentBase {
    unsafe fn get_object(agent: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(agent.battle_object)
    }

    unsafe fn get_boma(agent: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(agent.module_accessor)
    }
}