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
use super::*;
use crate::consts::globals::CMD_CAT1;
use smash::app;
use crate::utils::jump_checker_buffer;

static mut sonic_lightspeed_dash_frame_counter: [f32; 8] = [0.0; 8];
static mut SONIC_LIGHTSPEED_NO_JUMP: [bool; 8] = [false; 8];

unsafe extern "C" fn sonic_lightspeed_dash(fighter: &mut L2CFighterCommon) {

    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
	let situation_kind = StatusModule::situation_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if SONIC_LIGHTSPEED_NO_JUMP[id] && status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END && motion_kind != hash40("special_s_start") {
        SONIC_LIGHTSPEED_NO_JUMP[id] = false;
    }
    //JC the ending anim
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END && !SONIC_LIGHTSPEED_NO_JUMP[id] {
        enable_jump_cancel(boma, situation_kind, cat1, 0, 7);
    }

    if status_kind == *FIGHTER_STATUS_KIND_DEAD {
        sonic_lightspeed_dash_frame_counter[id] = 0.0;
        WorkModule::set_flag(boma, false, *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_FLAG);
    }

}

pub(crate) unsafe fn enable_jump_cancel(boma: &mut app::BattleObjectModuleAccessor, situation_kind: i32, cat1: i32, begin_frame_window: i32, end_frame_window: i32) {
    if jump_checker_buffer(boma, cat1) && MotionModule::frame(boma) >= begin_frame_window as f32 && MotionModule::frame(boma) <= end_frame_window as f32 {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            }
        }
        else if situation_kind == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }
    }
}

pub fn install() {
    Agent::new("sonic")
    .on_line(Main, sonic_lightspeed_dash)
    .install();
}