use smash::lib::lua_const::*;
use smash::phx::Vector2f;
use smashline::Main;
use smash::app::sv_system;
use std::convert::TryInto;
use smash::phx::*;
use smashline::Agent;
use smash::app::{self, lua_bind::*, utility, BattleObjectModuleAccessor};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::app::GroundCorrectKind;
use smashline::L2CValue;
use smash::app::FighterUtil::get_ground_correct_kind_air_trans;
use crate::utils::is_ready_go;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_DOUBLE_TRACTION;
use smash::app::sv_kinetic_energy::get_speed_x;
use crate::ext::BomaExt;
use crate::utils::compare_cat;
use crate::utils::*;
use smash::app::lua_bind::ControlModule;
use smash_script::lua_args;
use smash::lib::L2CAgent;

static mut double_traction_check: [bool; 9] = [true; 9];

//=================================================================
//== ECB SHIFTS
//=================================================================
unsafe extern "C" fn ecb(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = fighter.module_accessor;
        let status = StatusModule::status_kind(module_accessor);
        let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
        let situation = StatusModule::situation_kind(module_accessor);
        let kind = app::utility::get_kind(&mut *module_accessor);

        // Abort if game isn't active
        if !is_ready_go() {
            GroundModule::set_rhombus_offset(module_accessor, &Vector2f{x:0.0, y:0.0});
            return;
        }

        // Vanilla state filter
        let vanilla_ecb = [
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_THROWN,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_BURY,
            *FIGHTER_STATUS_KIND_BURY_WAIT,
        ].contains(&status);

        let previous_states = [
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_THROWN,
        ].contains(&prev_status);

        let fighter_low_offset = [
            *FIGHTER_KIND_KIRBY,
            *FIGHTER_KIND_PIKACHU,
            *FIGHTER_KIND_NESS,
            *FIGHTER_KIND_PURIN,
            *FIGHTER_KIND_GAMEWATCH,
            *FIGHTER_KIND_POPO,
            *FIGHTER_KIND_NANA,
            *FIGHTER_KIND_PICHU,
            *FIGHTER_KIND_METAKNIGHT,
            *FIGHTER_KIND_WARIO,
            *FIGHTER_KIND_PZENIGAME,
            *FIGHTER_KIND_PFUSHIGISOU,
            *FIGHTER_KIND_LUCAS,
            *FIGHTER_KIND_PIKMIN,
            *FIGHTER_KIND_TOONLINK,
            *FIGHTER_KIND_DUCKHUNT,
            *FIGHTER_KIND_MURABITO,
            *FIGHTER_KIND_INKLING,
            *FIGHTER_KIND_SHIZUE
        ];

        let fighter_mid_offset = [        
            *FIGHTER_KIND_MARIO,
            *FIGHTER_KIND_YOSHI,
            *FIGHTER_KIND_LUIGI,
            *FIGHTER_KIND_MARIOD,
            *FIGHTER_KIND_YOUNGLINK,
            *FIGHTER_KIND_PLIZARDON,
            *FIGHTER_KIND_DIDDY,
            *FIGHTER_KIND_DEDEDE,
            *FIGHTER_KIND_ROCKMAN,
            *FIGHTER_KIND_GEKKOUGA,
            *FIGHTER_KIND_PACMAN,
            *FIGHTER_KIND_KOOPAJR,
            *FIGHTER_KIND_PACKUN,
            *FIGHTER_KIND_MIIFIGHTER,
            *FIGHTER_KIND_MIISWORDSMAN,
            *FIGHTER_KIND_MIIGUNNER,
            *FIGHTER_KIND_PACKUN,
            *FIGHTER_KIND_BUDDY,
            *FIGHTER_KIND_PICKEL
        ];

        let fighter_high_offset = [
            *FIGHTER_KIND_FOX,
            *FIGHTER_KIND_FALCO,
            *FIGHTER_KIND_DAISY,
            *FIGHTER_KIND_MEWTWO,
            *FIGHTER_KIND_PIT,
            *FIGHTER_KIND_PITB,
            *FIGHTER_KIND_SONIC,
            *FIGHTER_KIND_SZEROSUIT,
            *FIGHTER_KIND_LUCARIO,
            *FIGHTER_KIND_ROBOT,
            *FIGHTER_KIND_WOLF,
            *FIGHTER_KIND_LITTLEMAC,
            *FIGHTER_KIND_KROOL,
            *FIGHTER_KIND_GAOGAEN
        ];

        let fighter_max_offset = [
            *FIGHTER_KIND_DONKEY,
            *FIGHTER_KIND_LINK,
            *FIGHTER_KIND_SAMUS,
            *FIGHTER_KIND_SAMUSD,
            *FIGHTER_KIND_CAPTAIN,
            *FIGHTER_KIND_PEACH,
            *FIGHTER_KIND_KOOPA,
            *FIGHTER_KIND_SHEIK,
            *FIGHTER_KIND_ZELDA,
            *FIGHTER_KIND_MARTH,
            *FIGHTER_KIND_LUCINA,
            *FIGHTER_KIND_GANON,
            *FIGHTER_KIND_ROY,
            *FIGHTER_KIND_CHROM,
            *FIGHTER_KIND_SNAKE,
            *FIGHTER_KIND_IKE,
            *FIGHTER_KIND_WIIFIT,
            *FIGHTER_KIND_ROSETTA,
            *FIGHTER_KIND_PALUTENA,
            *FIGHTER_KIND_REFLET,
            *FIGHTER_KIND_SHULK,
            *FIGHTER_KIND_RYU,
            *FIGHTER_KIND_KEN,
            *FIGHTER_KIND_CLOUD,
            *FIGHTER_KIND_KAMUI,
            *FIGHTER_KIND_BAYONETTA,
            *FIGHTER_KIND_RIDLEY,
            *FIGHTER_KIND_SIMON,
            *FIGHTER_KIND_RICHTER,
            *FIGHTER_KIND_JACK,
            *FIGHTER_KIND_BRAVE,
            *FIGHTER_KIND_DOLLY,
            *FIGHTER_KIND_MASTER,
            *FIGHTER_KIND_TANTAN,
            *FIGHTER_KIND_EDGE,
            *FIGHTER_KIND_EFLAME,
            *FIGHTER_KIND_ELIGHT,
            *FIGHTER_KIND_DEMON,
            *FIGHTER_KIND_TRAIL
        ];

        let offset_y = if fighter_low_offset.contains(&kind) {
            2.0
        } else if fighter_mid_offset.contains(&kind) {
            3.5
        } else if fighter_high_offset.contains(&kind) {
            4.0
        } else if fighter_max_offset.contains(&kind) {
            5.0
        } else {
            3.0 // fallback
        };

        // Skip states that should use vanilla ECB
        if vanilla_ecb || previous_states {
            return;
        }
        // Reset offset during ENTRY or early PASS (e.g. platform drop or training reset)
        let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
        let motion_frame = MotionModule::frame(module_accessor);

        if status == *FIGHTER_STATUS_KIND_ENTRY
            || (prev_status == *FIGHTER_STATUS_KIND_PASS && motion_frame < 3.0)
        {
            GroundModule::set_offset_y(module_accessor, 0.0);
            GroundModule::set_rhombus_offset(module_accessor, &Vector2f { x: 0.0, y: 0.0 });
            return;
        }

        // Check if fighter just entered the air
        let air_trans = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 10;

        // Only apply ECB logic for valid airborne movement states
        let valid_air_states = [
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        ];

        if situation == *SITUATION_KIND_AIR
            && valid_air_states.contains(&status)
            && !(vanilla_ecb || previous_states)
            && motion_frame > 2.0
        {
            let mut final_offset = offset_y;

            // Wavelanding ECB tweak: reduce ECB height for EscapeAir
            if status == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                final_offset -= 0.2;
            }

            GroundModule::set_offset_y(module_accessor, final_offset);

            if air_trans {
                GroundModule::set_rhombus_offset(module_accessor, &Vector2f { x: 0.0, y: 0.0 });
            }
        }

        // Reset offset on ground
        else if situation == *SITUATION_KIND_GROUND {
            GroundModule::set_offset_y(module_accessor, 0.0);

            if vanilla_ecb {
                GroundModule::set_rhombus_offset(module_accessor, &Vector2f { x: 0.0, y: 0.0 });
            }
        }
    }
}

#[skyline::hook(replace=get_ground_correct_kind_air_trans)]
unsafe extern "C" fn get_ground_correct_kind_air_trans_hook(_boma: &mut smash::app::BattleObjectModuleAccessor, _something: i32) -> i32 {
    return *GROUND_CORRECT_KIND_AIR;
}

//=================================================================
//== EXTRA TRACTION
//=================================================================
unsafe extern "C" fn extra_traction(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
	let facing = PostureModule::lr(boma);
    let max_walk: f32 = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
    let traction: f32 = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);

    let cur_speed = Vector2f {x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN),
                              y: KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN)};

    let travel_dir: f32 = if cur_speed.x >= 0.0 {1.0} else {-1.0};
    let kinetic_type: i32 = KineticModule::get_kinetic_type(boma);

    let mut set_brake = Vector3f {x: 0.0, y: 0.0, z: 0.0};

    //println!("Current Kinetic Type: {}", KineticModule::get_kinetic_type(boma));

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let test_speed = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    fighter.clear_lua_stack();

    // Extra traction when above max walk speed
        if cur_speed.x.abs() > max_walk && situation_kind == *SITUATION_KIND_GROUND {
            if [*FIGHTER_STATUS_KIND_WAIT,
                *FIGHTER_STATUS_KIND_LANDING_LIGHT,
                *FIGHTER_STATUS_KIND_LANDING,
                *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
                *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
                //*FIGHTER_STATUS_KIND_CATCH,
                *FIGHTER_STATUS_KIND_CATCH_PULL,
                *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                *FIGHTER_STATUS_KIND_WAIT,
                *FIGHTER_STATUS_KIND_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT_RV,
                *FIGHTER_STATUS_KIND_ATTACK,
                *FIGHTER_STATUS_KIND_ATTACK_S3,
                *FIGHTER_STATUS_KIND_ATTACK_HI3,
                *FIGHTER_STATUS_KIND_ATTACK_LW3,
                *FIGHTER_STATUS_KIND_ATTACK_S4_START,
                *FIGHTER_STATUS_KIND_ATTACK_S4,
                *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
                *FIGHTER_STATUS_KIND_ATTACK_HI4,
                *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
                *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
                    //double_traction_check[get_player_number(boma)] = true;
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_DOUBLE_TRACTION);
                }
            else{
                //double_traction_check[get_player_number(boma)] = false;
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_DOUBLE_TRACTION);
            }
        }
        else{
            //double_traction_check[get_player_number(boma)] = false;
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_DOUBLE_TRACTION);
        // Extra traction when above max walk speed
        if test_speed.abs() > max_walk {
            if [*FIGHTER_STATUS_KIND_WAIT,
                *FIGHTER_STATUS_KIND_LANDING_LIGHT,
                *FIGHTER_STATUS_KIND_LANDING,
                *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
                *FIGHTER_STATUS_KIND_CATCH,
                *FIGHTER_STATUS_KIND_CATCH_PULL,
                *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                *FIGHTER_STATUS_KIND_WAIT,
                *FIGHTER_STATUS_KIND_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT_RV,
                *FIGHTER_STATUS_KIND_ATTACK,
                *FIGHTER_STATUS_KIND_ATTACK_S3,
                *FIGHTER_STATUS_KIND_ATTACK_HI3,
                *FIGHTER_STATUS_KIND_ATTACK_LW3,
                *FIGHTER_STATUS_KIND_ATTACK_S4_START,
                *FIGHTER_STATUS_KIND_ATTACK_S4,
                *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
                *FIGHTER_STATUS_KIND_ATTACK_HI4,
                *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
                *FIGHTER_STATUS_KIND_ATTACK_LW4,
                *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) 
                && situation_kind == *SITUATION_KIND_GROUND {
                //set_brake.x = -traction * 1.0 * facing * travel_dir;
                //KineticModule::add_speed(boma, &set_brake);
                double_traction_check[get_player_number(boma)] = true;
            }
            else{
                double_traction_check[get_player_number(boma)] = false;
            }
        }
        else{
            double_traction_check[get_player_number(boma)] = false;
        }
        
        
        /*
        // Extra traction when above max walk speed
        if cur_speed.x.abs() > max_walk {
            if [*FIGHTER_STATUS_KIND_WAIT,
                *FIGHTER_STATUS_KIND_LANDING_LIGHT,
                *FIGHTER_STATUS_KIND_LANDING,
                *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
                *FIGHTER_STATUS_KIND_CATCH,
                *FIGHTER_STATUS_KIND_CATCH_PULL,
                *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                *FIGHTER_STATUS_KIND_WAIT,
                *FIGHTER_STATUS_KIND_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT_RV,
                *FIGHTER_STATUS_KIND_ATTACK,
                *FIGHTER_STATUS_KIND_ATTACK_S3,
                *FIGHTER_STATUS_KIND_ATTACK_HI3,
                *FIGHTER_STATUS_KIND_ATTACK_LW3,
                *FIGHTER_STATUS_KIND_ATTACK_S4_START,
                *FIGHTER_STATUS_KIND_ATTACK_S4,
                *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
                *FIGHTER_STATUS_KIND_ATTACK_HI4,
                *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
                *FIGHTER_STATUS_KIND_ATTACK_LW4,
                *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) 
                && situation_kind == *SITUATION_KIND_GROUND {
                set_brake.x = -traction * 1.0 * facing * travel_dir;
                KineticModule::add_speed(boma, &set_brake);
            }
        }
        */

        /*
        // Extra traction during DACDS until under 50% of walk speed
        if cur_speed.x.abs() > (max_walk * 0.5) {
            if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_START && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_DASH)
                || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 && StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_ATTACK_DASH) {
                set_brake.x = -traction * 0.9 * facing * travel_dir;
                KineticModule::add_speed(boma, &set_brake);
            }
        }
        */

        /*
        // Extra traction during damage until under 25% of walk speed
        if cur_speed.x.abs() > (max_walk * 0.25) {
            if((status_kind == *FIGHTER_STATUS_KIND_DAMAGE || status_kind == *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT) && situation_kind == *SITUATION_KIND_GROUND){
                set_brake.x = -traction * 0.75 * facing * travel_dir;
                KineticModule::add_speed(boma, &set_brake);
            }
        }
        */
        
    }
}


/*pub unsafe fn returnSmall(arg1: f32, arg2: f32) -> f32{
    if arg1 < arg2 {
        return arg1;
    }
    else {
        return arg2;
    }
}

pub unsafe fn returnLarge(arg1: f32, arg2: f32) -> f32{
    if arg1 > arg2 {
        return arg1;
    }
    else {
        return arg2;
    }
}

pub unsafe fn calcMomentum(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_mul = WorkModule::get_param_float(boma, hash40("jump_speed_x_mul"), 0);
    let stick_x = ControlModule::get_stick_x(boma);
    let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let calcJumpSpeed = (jump_speed_x * stick_x) + (jump_speed_x_mul * x_vel);
    let mut jumpSpeedClamped = 0.0;
    if x_vel < 0.0 {
        jumpSpeedClamped = returnLarge(calcJumpSpeed, -1.0 * jump_speed_x_max);
    }
    else {
        jumpSpeedClamped = returnSmall(calcJumpSpeed, jump_speed_x_max);
    }
    jumpSpeedClamped
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn status_attack_air_hook(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    let is_speed_backward = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma) < 0.0;
    let prev_status_check = [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(boma, 0));    
    let mut new_speed = CURRENTMOMENTUM[get_player_number(boma)];


        /*      Shorthop aerial macro and "bair stick flick" fix     */
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 && 
        StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT && !is_speed_backward { //if you used the shorthop aerial macro
        new_speed = calcMomentum(boma);
    }

    if prev_status_check {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(new_speed));
        smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }

    original!()(fighter, param_1)
}

pub unsafe fn additionalTransfer(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_FALL].contains(&status_kind) {
        CURRENTMOMENTUM[get_player_number(boma)] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
    }

            /*      ADDITIONAL MOVES THAT SHOULD CONSERVE MOMENTUM       */
    let mut should_conserve_momentum = false;
    
    if situation_kind == *SITUATION_KIND_AIR && MotionModule::frame(boma) <= 1.0 {

        if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_JACK, 
        *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_EDGE, *FIGHTER_KIND_LINK, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_DIDDY,
        *FIGHTER_KIND_ROY, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_REFLET, *FIGHTER_KIND_ELIGHT,
        *FIGHTER_KIND_LUCARIO, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_KROOL]
            .contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //put any fighter here whose neutral special should conserve momentum
                should_conserve_momentum = true; 
        }

        if should_conserve_momentum && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() > 0.1 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(CURRENTMOMENTUM[get_player_number(boma)]));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }

    }
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_sub)]
pub unsafe fn status_jump_sub_hook(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);

    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(calcMomentum(boma)));
    smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);


    original!()(fighter, param_2, param_3)
}

#[skyline::hook(replace = KineticModule::change_kinetic)]
pub unsafe fn change_kinetic_hook(boma: &mut smash::app::BattleObjectModuleAccessor, kinetic_type: i32) -> Option<i32> { //spacie laser momentum conservation
    let mut kinetic_type_new = kinetic_type;
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
    let mut should_change_kinetic = false;

    if [*FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_WOLF].contains(&fighter_kind) && status_kind == 446 /* laser status */ { 
        should_change_kinetic = true;
    }

    if [*FIGHTER_KINETIC_TYPE_FALL].contains(&kinetic_type_new) && should_change_kinetic {
        kinetic_type_new = -1;
    }     

    original!()(boma, kinetic_type_new)
}*/

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, ecb)
    //.on_line(Main, extra_traction)
	.install();
    skyline::install_hooks!(
        get_ground_correct_kind_air_trans_hook,
    );
}