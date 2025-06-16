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
use smash::app::sv_animcmd::EFFECT_FOLLOW_RND;
use crate::ext::BomaExt;
use smash::app::BattleObjectModuleAccessor;
use smash::app;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL_USED;
use crate::consts::globals::CMD_CAT1;
use smash::app::lua_bind::StatusModule::situation_kind;
use crate::utils;
use crate::utils::jump_checker_buffer;
use crate::utils::compare_cat;

pub(crate) fn is_jc(boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind : i32, status_kind : i32, frame : f32) -> bool {
	unsafe {
		//[fighter_kind, status_kind, hit_condition, jc_start, jc_end]
		let jump_cancel = [ [*FIGHTER_KIND_LUCINA, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, 0, -1, -1] ];
		for i in &jump_cancel {
			if fighter_kind == i[0] && status_kind == i[1] {
				println!("jc status");
				if i[3] != -1 && i[4] != -1 {
					if (frame as i32) < i[3] || (frame as i32) >= i[4] {
						continue;
					};
				};
				if i[2] != 0 {
					if AttackModule::is_infliction_status(boma, i[2]) {
						return true;
					};
				} else {
					return true;
				};
			};
		};
		return false;
	}
}

pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}

unsafe extern "C" fn s4_jump_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {	
		let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(boma);
		if is_jc(boma, fighter_kind, status_kind, frame) && check_jump(boma){
				if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
				};
		};
    }
}

unsafe extern "C" fn side_special_cancels(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let motion_kind = MotionModule::motion_kind(boma);

  
    let fighter_kind = smash::app::utility::get_kind(boma);
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(boma);



    if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            // Up
            if [hash40("special_s3_hi"), hash40("special_air_s3_hi")].contains(&motion_kind) {
                // Check for tilt attack inputs
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
                // Check for smash attack inputs
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
            }
            // Forward
            if [hash40("special_s3_s"), hash40("special_air_s3_s")].contains(&motion_kind) {
                // Check for tilt attack inputs
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
                // Check for smash attack inputs
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
            }
            // Down
            if [hash40("special_s3_lw"), hash40("special_air_s3_lw")].contains(&motion_kind) {
                // Check for tilt attack inputs
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
                // Check for smash attack inputs
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                    }
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn dancing_blade_vertical_momentum(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_gravity = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2]) && boma.is_situation(*SITUATION_KIND_AIR) {
        lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.06);
        lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -1.5);
    }

    if boma.is_situation(*SITUATION_KIND_GROUND) && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL_USED) { 
        WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL_USED);
    }
}

unsafe extern "C" fn soaring_slash_drift(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let stick_x = ControlModule::get_stick_x(boma);
    
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            if stick_x != 0.0 {
                let motion_vec = utils::x_motion_vec(0.4, stick_x);
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }
    }
}

// Lengthen swords
unsafe extern "C" fn sword_length(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let long_sword_scale = Vector3f{x: 1.015, y: 1.115, z: 1.045};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}

pub fn install() {
    Agent::new("chrom")
    .on_line(Main, s4_jump_cancel)
    .on_line(Main, side_special_cancels)
    .on_line(Main, dancing_blade_vertical_momentum)
    .on_line(Main, soaring_slash_drift)
    .on_line(Main, sword_length)
    .install();
}

/*static mut id: [bool; 8] = [false; 8];
static mut soaring_slash_hit: [bool; 8] = [false; 8];

unsafe extern "C" fn soaring_slash(fighter: &mut L2CFighterCommon) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2].contains(&status_kind) {
        if AttackModule::is_infliction(boma, 2) {
            soaring_slash_hit[id] = true;
        }
    }
    if ![*FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) {
        soaring_slash_hit[id] = false;
    }
}*/