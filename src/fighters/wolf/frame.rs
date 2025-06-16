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
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::consts::globals::CMD_CAT1;
use crate::utils;
use crate::utils::compare_cat;
use crate::ext::BomaExt;

unsafe extern "C" fn airdodge_cancel(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let situation_kind = StatusModule::situation_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
	let frame = MotionModule::frame(boma);

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if frame > 17.0 {
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }
    }
}

// Wolf Shine Jump Cancels
unsafe extern "C" fn shine_jump_cancel(fighter: &mut L2CFighterCommon) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let situation_kind = StatusModule::situation_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
	
	if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if utils::jump_checker_buffer(boma, cat1) {
            if situation_kind == *SITUATION_KIND_AIR {
                if boma.get_jump_count() < boma.get_jump_count_max() {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            } else if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
    }
}

unsafe extern "C" fn fang_cancel(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let frame = MotionModule::frame(boma);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            if frame > 20.0 {
                CancelModule::enable_cancel(boma);
            }
            if frame >= MotionModule::end_frame(boma) - 3.0{
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}


pub fn install() {
    Agent::new("wolf")
	.on_line(Main, airdodge_cancel)
	.on_line(Main, shine_jump_cancel)
    .on_line(Main, fang_cancel)
    .install();
}