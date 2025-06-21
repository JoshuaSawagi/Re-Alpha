use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
//
use smash::app::*;
use smash::phx::Vector3f;
use smash::app::sv_animcmd::EFFECT_FOLLOW_RND;
use crate::ext::BomaExt;
use smash::app::BattleObjectModuleAccessor;
use smash::app;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL_USED;
use smash::app::lua_bind::ModelModule;
use smashline::L2CFighterCommon;

unsafe extern "C" fn up_special_reverse(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let facing = PostureModule::lr(boma); // <-- Needed for smash_turn

    // Marth/Lucina frame 6
    // Roy frame 10
    let mut target_frame = 6.0;
    if fighter_kind == *FIGHTER_KIND_MARTH {
        target_frame = 10.0;
    }

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if frame == target_frame {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

unsafe extern "C" fn dancing_blade_vertical_momentum(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_gravity = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2]) && boma.is_situation(*SITUATION_KIND_AIR) {
        lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.06);
        lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -1.5);
    }

    if boma.is_situation(*SITUATION_KIND_GROUND) && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL_USED) { 
        WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL_USED);
    }
}

// Lengthen swords
unsafe extern "C" fn sword_length(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let long_sword_scale = Vector3f{x: 1.015, y: 1.115, z: 1.045};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}

pub fn install() {
    Agent::new("marth")
    .on_line(Main, up_special_reverse)
    .on_line(Main, dancing_blade_vertical_momentum)
    .on_line(Main, sword_length)
    .install();
}