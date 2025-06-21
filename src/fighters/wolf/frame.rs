use smash::lib::lua_const::*;
use smash::phx::{Vector2f, Vector3f};
use smash::app::sv_system;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use crate::ext::Vec3Ext;
use crate::ext::Vec2Ext;
use smash::app::GroundCorrectKind;
use smash::hash40;
use crate::consts::globals::CMD_CAT1;
use crate::consts::globals::CMD_CAT2;
use crate::utils::compare_cat;
use crate::utils;
use crate::utils::jump_checker_buffer;
use crate::ext::BomaExt;

static mut illusion_shorten: [bool; 8] = [false; 8];
static mut illusion_shortened: [bool; 8] = [false; 8];

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

unsafe extern "C" fn illusion_short(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);

    if motion_kind == hash40("special_s") || motion_kind == hash40("special_air_s") {
        if frame <= 1.0 {
            illusion_shorten[id] = false;
            illusion_shortened[id] = false;
        }
        if illusion_shorten[id] && !illusion_shortened[id] {
            let motion_vec = Vector3f{x: 0.25, y: 1.0, z: 1.0};
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            illusion_shortened[id] = true;
        }
        if compare_cat(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) && !illusion_shortened[id] {
            illusion_shorten[id] = true;
            WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
        }
    }
}

// Side Special Cancels
unsafe extern "C" fn  side_special_cancels(fighter: &mut L2CFighterCommon) {
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
    .on_line(Main, illusion_short)
    .on_line(Main, side_special_cancels)
    .install();
}