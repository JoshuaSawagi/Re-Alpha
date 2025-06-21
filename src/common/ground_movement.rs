use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use std::{fs, path::Path};
use smash::phx::Vector2f;
use smash::phx::Vector3f;
use smash::app::sv_system;
use smash::app;
use crate::common::ground_movement::PostureModule::lr;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_FLAG_MOONWALK;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_FLAG_MOONWALK_JUMP;
use crate::utils::clamp;
use crate::consts::globals::STICK_X;
use crate::utils::get_player_number;


const PIVOT_STICK_SNAPBACK_WINDOW: f32 = 2.0;
unsafe extern "C" fn pivot(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
        let prev_status_kind_1 = StatusModule::prev_status_kind(boma, 1);
        let curr_frame = MotionModule::frame(boma);
        let stick_value_x = ControlModule::get_stick_x(boma);
        let facing = PostureModule::lr(boma);

        let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
        let mut pivot_boost: Vector3f = Vector3f { x: dash_speed * 0.70, y: 0.0, z: 0.0, };

        if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH && curr_frame <= PIVOT_STICK_SNAPBACK_WINDOW
            && stick_value_x == 0.0
            && [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&prev_status_kind)
            && ![*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&prev_status_kind_1) {
            
            // Optional tweak
            if curr_frame == 3.0 {
                pivot_boost.x = dash_speed * 0.35;
            }

            PostureModule::reverse_lr(boma);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
            KineticModule::clear_speed_all(boma);
            KineticModule::add_speed(boma, &pivot_boost);
        }
    }
}

//=================================================================
//== DASH DROP
//=================================================================
unsafe extern "C" fn dash_drop(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE,
            * FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
            if GroundModule::is_passable_ground(boma) && stick_y < -0.66 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, false);
            }
        }
    }
}

//=================================================================
//== CROUCH DURING RUN
//=================================================================
unsafe extern "C" fn run_squat(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        //let crouch_thresh: f32 = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
        if status_kind == *FIGHTER_STATUS_KIND_RUN || status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
            if stick_y < -0.66 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }
    }
}

//=================================================================
//== RESPAWN_TAUNT
//=================================================================
unsafe extern "C" fn respawn_taunt(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
            let motion_kind = MotionModule::motion_kind(boma);
            if motion_kind == hash40("appeal_hi_r") ||
                motion_kind == hash40("appeal_hi_l") ||
                motion_kind == hash40("appeal_lw_r") ||
                motion_kind == hash40("appeal_lw_l") ||
                motion_kind == hash40("appeal_s_l") ||
                motion_kind == hash40("appeal_s_r") {
                return;
            }

            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                if PostureModule::lr(boma) == 1.0 {
                    MotionModule::change_motion(boma, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(boma, Hash40::new("appeal_hi_l"), 0.0, 1.0, false, 0.0, false, false);
                }
            }

            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if PostureModule::lr(boma) == 1.0 {
                    MotionModule::change_motion(boma, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(boma, Hash40::new("appeal_lw_l"), 0.0, 1.0, false, 0.0, false, false);
                }
            }

            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if PostureModule::lr(boma) == 1.0 {
                    MotionModule::change_motion(boma, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(boma, Hash40::new("appeal_s_l"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
}


pub static mut SPEED_X : [f32; 8] = [0.0; 8];
pub(crate) unsafe fn get_speed_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return SPEED_X[ENTRY_ID]
}

//=================================================================
//== MOONWALK
//=================================================================
unsafe extern "C" fn moonwalk(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let mut stickx = ControlModule::get_stick_x(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lr = PostureModule::lr(boma);
        let walk_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_add"), 0);
        let walk_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_mul"), 0);
        let walk_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_speed_max"), 0);
        let max_moonwalk = walk_speed_max * 1.2;
		stickx = stickx * lr;
        let mw_modifier = 1.1;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind){
			if stickx < -0.2 {
                let moonwalk_speed = (stickx*walk_accel_mul - walk_accel_add)*mw_modifier;
                if (get_speed_x(boma)*lr)+moonwalk_speed > -max_moonwalk {
                    let speed = smash::phx::Vector3f { x: moonwalk_speed, y: 0.0, z: 0.0 };
                    KineticModule::add_speed(boma, &speed);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
                } else {
                    let current_back = get_speed_x(boma)*lr;
                    let speed = smash::phx::Vector3f { x:-(current_back+max_moonwalk), y: 0.0, z: 0.0 };
                    KineticModule::add_speed(boma, &speed);
                }
            }
		};
    };
}

/*
static mut moonwalk_mul: [f32;8] = [1.0;8];
unsafe extern "C" fn moonwalk(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let stick_value_x = ControlModule::get_stick_x(boma);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let facing = PostureModule::lr(boma);
        /* Moonwalk melee calculation: (stick_pos.x * run_accel_mul) + (sign(stick_pos.x) * run_accel_add) */
        let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let stick_pos_x = ControlModule::get_stick_x(boma);
        let run_accel_add = WorkModule::get_param_float(boma, hash40("run_accel_add"), 0);
        let run_accel_mul = WorkModule::get_param_float(boma, hash40("run_accel_mul"), 0);

        if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind) && stick_value_x*facing < 0.0 {
            let moonwalk_vel: f32 = ((stick_pos_x * run_accel_mul) + (stick_pos_x / stick_pos_x.abs() * run_accel_add)).abs() * moonwalk_mul[get_player_number(boma)];
            let added_speed = Vector3f { x: -moonwalk_vel, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &added_speed);
            moonwalk_mul[get_player_number(boma)] += 0.05;
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
        }else{
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && 
            [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(boma,0)) && moonwalk_mul[get_player_number(boma)] != 1.0 {
                if x_vel < 0.0 {
                    PostureModule::set_lr(boma, 1.0);
                }else{
                    PostureModule::set_lr(boma, -1.0);
                }
                PostureModule::update_rot_y_lr(boma);
            }
        }

        if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && moonwalk_mul[get_player_number(boma)] != 1.0 {
            moonwalk_mul[get_player_number(boma)] = 1.0;
        }
    }
}*/

pub fn install() {
    Agent::new("fighter")
    .on_line(Main, pivot)
    .on_line(Main, dash_drop)
    //.on_line(Main, fixbackdash)
    .on_line(Main, run_squat)
    .on_line(Main, respawn_taunt)
    .on_line(Main, moonwalk)
	.install();
}