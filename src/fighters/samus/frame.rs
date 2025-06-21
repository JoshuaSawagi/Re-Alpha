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
use crate::consts::globals::CMD_CAT1;
use crate::consts::globals::CMD_CAT2;
use crate::utils::compare_cat;

unsafe extern "C" fn missle_cancel(fighter: &mut L2CFighterCommon) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let situation_kind = StatusModule::situation_kind(boma);
    let cat2 = fighter.global_table[CMD_CAT2].get_i32();
	let stick_y = ControlModule::get_stick_y(boma);

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_LANDING, true);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

pub fn install() {
	Agent::new("samus")
	.on_line(Main, missle_cancel)
	.install();
}
	