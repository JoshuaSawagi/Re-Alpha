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

unsafe extern "C" fn laser_land_cancel(fighter: &mut L2CFighterCommon) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let situation_kind = StatusModule::situation_kind(boma);
    let cat2 = fighter.global_table[CMD_CAT2].get_i32();
	let stick_y = ControlModule::get_stick_y(boma);

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
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

unsafe extern "C" fn shine_jc_turnaround(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let situation_kind = StatusModule::situation_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
	let stick_x = ControlModule::get_stick_x(boma);
    let facing = PostureModule::lr(boma);
	let frame = MotionModule::frame(boma);

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if stick_x * facing < 0.0 {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
        if frame > 3.0 {
            KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            if utils::jump_checker_buffer(boma, cat1) {
                if situation_kind == *SITUATION_KIND_AIR {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

pub fn install() {
	Agent::new("falco")
	.on_line(Main, laser_land_cancel)
	.on_line(Main, shine_jc_turnaround)
	.install();
}