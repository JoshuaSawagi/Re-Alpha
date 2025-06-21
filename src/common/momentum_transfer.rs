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
use smash::app::sv_kinetic_energy;
use crate::utils::get_player_number;
use crate::utils::clamp;
use crate::StatusModule::situation_kind;
use crate::get_category;

const WORK_ID_ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x1000;
const WORK_ID_JUMP_SPEED_RATIO: i32 = 0x1001;
const WORK_ID_JUMP_SQUAT_FRAME: i32 = 0x1002;
const WORK_ID_JUMPSQUAT_VELOCITY: i32 = 0x1003;
const WORK_ID_CURRENT_MOMENTUM: i32 = 0x1005;
const WORK_ID_CURRENT_MOMENTUM_SPECIALS: i32 = 0x1006;

pub static mut rar_leniency:           [f32;8]  = [0.0;8];
pub static mut ground_vel:           [f32;8]  = [0.0;8];
pub static mut curr_momentum: [f32;8] = [0.0;8];

//Jump (runs once at the beginning of the status)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_sub)]
pub unsafe fn status_jump_sub_hook(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue) -> L2CValue {
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);

    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(calc_melee_momentum(boma)));
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);


    original!()(fighter, param_2, param_3)
}


//Aerials (runs once at the beginning of the status)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn status_attack_air_hook(fighter: &mut L2CFighterCommon, param_1: L2CValue){
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    let is_speed_backward = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma) < 0.0;
    let prev_status_check = [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(boma, 0));    
    let mut new_speed = curr_momentum[get_player_number(boma)];


        /*      Shorthop aerial macro and "bair stick flick" fix     */
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 && 
        StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT && !is_speed_backward { //if you used the shorthop aerial macro
        new_speed = calc_melee_momentum(boma);
    }

    if prev_status_check {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(new_speed));
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }

    original!()(fighter, param_1)
}


//called in moveset_edits in sys_line_system_control_fighter.rs
pub unsafe fn momentum_transfer_helper(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, curr_frame: f32, fighter_kind: i32) {
    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_FALL].contains(&status_kind) {
        curr_momentum[get_player_number(boma)] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
    }
}



unsafe fn calc_melee_momentum(boma: &mut app::BattleObjectModuleAccessor) -> f32 {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_mul = WorkModule::get_param_float(boma, hash40("jump_speed_x_mul"), 0);
    let stick_x = ControlModule::get_stick_x(boma);
    let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);

    let calcJumpSpeed = (jump_speed_x * stick_x) + (jump_speed_x_mul * x_vel);
    let jumpSpeedClamped = clamp(calcJumpSpeed, -jump_speed_x_max, jump_speed_x_max);  //melee jump speed calculation... courtesey of Brawltendo
    jumpSpeedClamped
}



#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sys_line_system_control_fighter)]
pub fn sys_line_system_control_fighter(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
        let lua_state = fighter.lua_state_agent;
        let battle_object_category = get_category(boma);

        if battle_object_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
            let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(boma) as i32;
            let curr_frame = smash::app::lua_bind::MotionModule::frame(boma);
            let fighter_kind = get_kind(boma);
            momentum_transfer_helper(
                lua_state, &mut l2c_agent, boma,
                status_kind, situation_kind, curr_frame, fighter_kind,
            );
        }
    }
}


// Hook installed only when common is loaded
fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
                status_jump_sub_hook,
                status_attack_air_hook,
                sys_line_system_control_fighter
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}