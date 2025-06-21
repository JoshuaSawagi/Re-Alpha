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
use smash::app::*;
use smash::phx::Vector3f;
use smash::app::BattleObjectModuleAccessor;
use smash::app;
use crate::utils::compare_cat;
use crate::consts::globals::CMD_CAT2;

// Young Link Fire Arrow fast fall
unsafe extern "C" fn arrow_fastfall(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let stick_y = ControlModule::get_stick_y(boma);
    let cat2 = fighter.global_table[CMD_CAT2].get_i32();

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Lengthen swords
unsafe extern "C" fn sword_length(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let long_sword_scale = Vector3f{x: 1.0, y: 1.1, z: 1.0};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword"), &long_sword_scale);
}

pub fn install() {
    Agent::new("younglink")
    .on_line(Main, arrow_fastfall)
    .on_line(Main, sword_length)
    .install();
}