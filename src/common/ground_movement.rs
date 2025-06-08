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
use smash::app::sv_system;
use smash::app;
use crate::common::ground_movement::PostureModule::lr;
use smash::app::lua_bind::KineticEnergy::get_speed_x;

const PIVOT_STICK_SNAPBACK_WINDOW: f32 = 2.0;

unsafe extern "C" fn pivot(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
        let prev_status_kind_1 = StatusModule::prev_status_kind(boma, 1);
        let curr_frame = MotionModule::frame(boma);
        let stick_value_x = ControlModule::get_stick_x(boma);
        let lr = PostureModule::lr(boma);

        let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
        let mut pivot_boost: Vector3f = Vector3f { x: dash_speed * 0.70, y: 0.0, z: 0.0, };

        if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
            && curr_frame <= PIVOT_STICK_SNAPBACK_WINDOW
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
unsafe extern "C" fn dash_drop(fighter : &mut L2CFighterCommon) {
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


// Moonwalk (Enhanced Melee-style)
/*static mut MOONWALK_MUL: [f32; 8] = [1.0; 8];

unsafe extern "C" fn moonwalk(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let stick_x = ControlModule::get_stick_x(boma);
    let status_kind = StatusModule::status_kind(boma);
    let lr = PostureModule::lr(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    // Run acceleration values (not walk)
    let run_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_add"), 0);
    let run_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_mul"), 0);
    let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
    let stick_x_facing = stick_x * lr;

    // Melee-style moonwalk conditions
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && stick_x_facing < -0.2 {
        // Apply Melee moonwalk formula with scaling
        let base_moonwalk_speed = ((stick_x * run_accel_mul) + (stick_x.signum() * run_accel_add)).abs();
        let moonwalk_speed = -base_moonwalk_speed * MOONWALK_MUL[entry_id];

        // Cap moonwalk speed
        let max_moonwalk = run_speed_max * 1.8;

        let kinetic = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let current_speed = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent) * lr;
        if (current_speed + moonwalk_speed.abs()) < max_moonwalk {
            let speed = smash::phx::Vector3f { x: moonwalk_speed, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &speed);
        } else {
            let capped = -((current_speed + max_moonwalk) * lr);
            let speed = smash::phx::Vector3f { x: capped, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &speed);
        }

        // Increase slide over time
        MOONWALK_MUL[entry_id] += 0.05;

        // Prevent turning
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    }

    // Direction fix when jumping out of moonwalk
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT
        && [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(boma, 0))
        && MOONWALK_MUL[entry_id] != 1.0
    {
        let kinetic = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let x_vel =  app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent) * kinetic;

        if x_vel < 0.0 {
            PostureModule::set_lr(boma, 1.0);
        } else {
            PostureModule::set_lr(boma, -1.0);
        }
        PostureModule::update_rot_y_lr(boma);
    }

    // Reset moonwalk multiplier when not in dash states
    if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind)
        && MOONWALK_MUL[entry_id] != 1.0
    {
        MOONWALK_MUL[entry_id] = 1.0;
    }
}*/


pub fn install() {
    Agent::new("fighter")
    .on_line(Main, pivot)
    .on_line(Main, dash_drop)
    .on_line(Main, run_squat)
    .on_line(Main, respawn_taunt)
	.install();
}
