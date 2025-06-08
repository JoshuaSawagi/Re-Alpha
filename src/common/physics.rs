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
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
        let max_walk = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
        if speed_x.abs() > max_walk
        && boma.is_situation(*SITUATION_KIND_GROUND)
        && boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
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
            *FIGHTER_STATUS_KIND_ATTACK_LW4
        ]) 
        {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_DOUBLE_TRACTION);
        } else {
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_DOUBLE_TRACTION);
        }
    }
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, ecb)
    .on_line(Main, extra_traction)
	.install();
    skyline::install_hooks!(
        get_ground_correct_kind_air_trans_hook,
    );
}