use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::phx::Vector2f;
use crate::FIGHTER_MANAGER;
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use std::{fs, path::Path};
use crate::controls::ext::*;
use crate::common::*;

static mut STATUS_DURATION : [i32; 8] = [0; 8];
static mut MOTION_DURATION : [i32; 8] = [0; 8];
pub static mut POS_X : [f32; 8] = [0.0; 8];
pub static mut POS_Y : [f32; 8] = [0.0; 8];
pub static mut STICK_DIR: [f32; 8] = [0.0; 8];
pub static mut PREV_SPEED_X : [f32; 8] = [0.0; 8];
pub static mut PREV_SPEED_Y : [f32; 8] = [0.0; 8];
pub static mut SPEED_X : [f32; 8] = [0.0; 8];
pub static mut SPEED_Y : [f32; 8] = [0.0; 8];



//Position and speed
pub(crate) unsafe fn ray_check_pos(boma: &mut smash::app::BattleObjectModuleAccessor, x_distance : f32, y_distance: f32, ignore_plat: bool) -> u64 {
	GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: x_distance, y: y_distance}, ignore_plat)
}
pub(crate) unsafe fn get_prev_speed_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return PREV_SPEED_X[ENTRY_ID]
}
pub(crate) unsafe fn get_prev_speed_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return PREV_SPEED_Y[ENTRY_ID]
}
pub(crate) unsafe fn get_speed_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return SPEED_X[ENTRY_ID]
}
pub(crate) unsafe fn get_speed_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return SPEED_Y[ENTRY_ID]
}
pub(crate) unsafe fn get_accel_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return ACCEL_X[ENTRY_ID]
}
pub(crate) unsafe fn get_accel_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return ACCEL_X[ENTRY_ID]
}
pub(crate) unsafe fn get_to_run_flag(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return TO_RUN_FLAG[ENTRY_ID]
}

//Hitlag and Hitstun
pub(crate) unsafe fn is_hitlag(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
		return false
	} else {
		return true
	}
}
pub(crate) unsafe fn get_hitlag(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME)
}
pub(crate) unsafe fn get_stick_angle(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	println!("stick dir: {}", STICK_DIR[ENTRY_ID]);
	return STICK_DIR[ENTRY_ID]
}

//Misc.
pub(crate) unsafe fn is_reset() -> bool {
	if smash::app::sv_information::is_ready_go() {
		return false
	} else {
		return true
	}
}

pub(crate) unsafe fn is_default(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) < 16  {
		return true 
	} else {
		return false
	}
    
}
pub(crate) unsafe fn is_added(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127)  {
		return true 
	} else {
		return false
	}
}
pub(crate) unsafe fn set_knockdown_throw(fighter: &mut L2CAgentBase) -> () {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
	let opponent_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
	let grabber_boma = smash::app::sv_battle_object::module_accessor(opponent_id);
	let grabber_kind = smash::app::utility::get_kind(&mut *grabber_boma);
	let grabber_entry_id = WorkModule::get_int(&mut *grabber_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	IS_KD_THROW[grabber_entry_id] = true;
}

pub(crate) unsafe fn is_tap_djc(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return TAP_JUMP_BUFFER[ENTRY_ID] <= 0;
}
