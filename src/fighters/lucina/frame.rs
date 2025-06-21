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
//
use smash::app::*;
use smash::phx::Vector3f;
use smash::app::sv_animcmd::EFFECT_FOLLOW_RND;
use crate::ext::BomaExt;
use smash::app::BattleObjectModuleAccessor;
use smash::app;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL_USED;
use crate::utils::compare_cat;
use crate::utils::jump_checker_buffer;
use crate::consts::globals::CMD_CAT1;

unsafe extern "C" fn side_special_cancels(fighter: &mut L2CFighterCommon) {
    unsafe {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let motion_kind = MotionModule::motion_kind(boma);
    let fighter_kind = smash::app::utility::get_kind(boma);
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
        // Jump cancel 4th hit high
        if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if [hash40("special_s4_hi"), hash40("special_air_s4_hi")].contains(&motion_kind) && MotionModule::frame(boma) > 13.0 {
                    if jump_checker_buffer(boma, cat1) {
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
        }
    }
}

// Up Special Reverse
unsafe extern "C" fn up_special_reverse(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let facing = PostureModule::lr(boma);

    // Marth/Lucina frame 6
    // Roy frame 10
    let mut target_frame = 6.0;
    if fighter_kind == *FIGHTER_KIND_LUCINA {
        target_frame = 6.0;
    }

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if frame == target_frame {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
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

// Lengthen swords
unsafe extern "C" fn sword_length(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let long_sword_scale = Vector3f{x: 1.015, y: 1.115, z: 1.045};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}

pub fn install() {
    Agent::new("lucina")

    .on_line(Main, side_special_cancels)
    .on_line(Main, up_special_reverse)
    .on_line(Main, dancing_blade_vertical_momentum)
    .on_line(Main, sword_length)

    .install();
}