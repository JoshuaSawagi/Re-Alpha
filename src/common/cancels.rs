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
use smash::app::sv_system;
use smash::app;
use crate::utils::compare_cat;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::phx::Hash40;
use crate::utils::get_player_number;

pub const CMD_CAT1:              i32 = 0x20;
pub const CMD_CAT2:              i32 = 0x21;
pub const CMD_CAT3:              i32 = 0x22;

// Define for all 8 players (assuming 8-player support)
static mut popo_jc_grab: [bool; 8] = [false; 8];
static mut ditcit_sliding: [bool; 8] = [false; 8];

//=================================================================
//== JUMP CANCEL GRABS
//=================================================================
unsafe extern "C" fn jump_cancel_grab(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) {
            if fighter_kind == *FIGHTER_KIND_POPO {
                popo_jc_grab[get_player_number(boma)] = true;
            }
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
        }
    }
}

//=================================================================
//== DITCIT
//=================================================================
unsafe extern "C" fn ditcit(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let facing = PostureModule::lr(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let player_number = get_player_number(boma);
    let mut motion_value: f32 = 0.0;
    let mut motion_vec = Vector3f {x: 0.0, y: 0.0, z: 0.0};

    if status_kind != *FIGHTER_STATUS_KIND_ITEM_THROW {
        ditcit_sliding[player_number] = false;
    }

    if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW_DASH {
        if MotionModule::frame(boma) > 2.0 && MotionModule::frame(boma) < 6.0
            && ((compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4))
             || (compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4))
             || (compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4))
             || (compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3))
             || (compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3))
             || (compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3))) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
            ditcit_sliding[player_number] = true;
        }
    } else {
        if ditcit_sliding[player_number] {  // status_kind == ITEM_THROWN, coming from THROW_DASH
            motion_value = 2.8 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
            motion_vec.x = motion_value * facing;
            motion_vec.y = 0.0;
            motion_vec.z = 0.0;
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

//=================================================================
//== DACUS
//=================================================================
unsafe extern "C" fn dacus(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let stick_y = ControlModule::get_stick_y(boma);

    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if MotionModule::frame(boma) < 10.0 {
            let is_catch = compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_CATCH);

            // Normal smash input or Z with left stick
            if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) || (stick_y >= 0.7 && is_catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }

            if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) || (stick_y <= -0.7 && is_catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }

            // Adjust input window of tilts to prevent accidental smashes
            if MotionModule::frame(boma) > 2.0 {
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                }
                if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
                }
            }
        }
    }
}

//=================================================================
//== JUMP CANCEL AIRDODGE
//=================================================================
unsafe extern "C" fn jump_cancel_airdodge(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) && !compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
    }
}

//=================================================================
//== AIRDODGE CANCEL ZAIR AND ITEM TOSS
//=================================================================
unsafe extern "C" fn airdodge_cancels(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let facing = PostureModule::lr(boma);
    let cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let cat3 = fighter.global_table[CMD_CAT3].get_i32();
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        if MotionModule::frame(boma) > 3.0 && MotionModule::frame(boma) < 41.0 {
            // Throw item
            if ItemModule::is_have_item(boma, 0) {
                if compare_cat(cat3, *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_ALL) {
                    if facing * stick_x < 0.0 {
                        PostureModule::reverse_lr(boma);
                    }
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
                }
            } else { // Zair if no item toss
                if [*FIGHTER_KIND_LUCAS,
                    *FIGHTER_KIND_YOUNGLINK,
                    *FIGHTER_KIND_TOONLINK,
                    *FIGHTER_KIND_SAMUS,
                    *FIGHTER_KIND_SAMUSD,
                    *FIGHTER_KIND_SZEROSUIT,
                    *FIGHTER_KIND_LUIGI].contains(&fighter_kind) {
                    if !ItemModule::is_have_item(boma, 0) {
                       if compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_AIR_LASSO) {
                           StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_AIR_LASSO, true);
                       }
                    }
                }
            }
        }
    }
}

//=================================================================
//== ANTI-FOOTSTOOL DEGENERACY TECH
//=================================================================
static mut footstool_airdodge_lockout: [bool; 8] = [false; 8];

unsafe extern "C" fn footstool_defense(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    // Shield cancel grounded footstool recoil after being ground footstooled and then receiving
    // histun beforehand
    let prev_status_0 = StatusModule::prev_status_kind(boma, 0);
    let prev_status_1 = StatusModule::prev_status_kind(boma, 1);
    let prev_status_2 = StatusModule::prev_status_kind(boma, 2);
    let prev_status_3 = StatusModule::prev_status_kind(boma, 3);
    if (status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV && situation_kind == *SITUATION_KIND_GROUND)
        && (prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE)
          || (prev_status_2 == *FIGHTER_STATUS_KIND_DAMAGE_AIR && prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE)
          || (prev_status_3 == *FIGHTER_STATUS_KIND_DAMAGE && prev_status_2 == *FIGHTER_STATUS_KIND_DAMAGE_AIR && prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV {
        // DamageModule::add_damage(boma, 100.0, 0);
    }

    let player_number = get_player_number(boma);

    // Prevent airdodging after a footstool until after F20
    if (status_kind == *FIGHTER_STATUS_KIND_JUMP && prev_status_0 == *FIGHTER_STATUS_KIND_TREAD_JUMP)
        || (status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL && prev_status_0 == *FIGHTER_STATUS_KIND_JUMP && prev_status_1 == *FIGHTER_STATUS_KIND_TREAD_JUMP)
        && MotionModule::frame(boma) < 20.0 {
        footstool_airdodge_lockout[player_number] = true;
    } else if footstool_airdodge_lockout[player_number] {
        footstool_airdodge_lockout[player_number] = false;
    }
}



pub fn install() {
    Agent::new("fighter")
    .on_line(Main, jump_cancel_grab)
    .on_line(Main, ditcit)
    .on_line(Main, dacus)
    .on_line(Main, jump_cancel_airdodge)
    .on_line(Main, airdodge_cancels)
    .on_line(Main, footstool_defense)
	.install();
}
