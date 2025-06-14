use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::L2CValue;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::app::SituationKind;
use super::*;
use smash::phx::Vector3f;
use smash::app::sv_math;
use std::convert::TryInto;
use smash::app;
use smash::app::GroundCorrectKind;
use smash::phx::Vector2f;
use smash::app::KineticUtility;
use smash::lua2cpp::L2CFighterCommon_status_end_JumpSquat;
use smash::lua2cpp::L2CFighterCommon_status_JumpSquat_Main;
use crate::consts::globals::SITUATION_KIND;
use crate::consts::globals::CMD_CAT1;
use crate::consts::globals::CMD_CAT2;
use crate::consts::globals::CHECK_ATTACK_HI4_UNIQ;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY;
use crate::consts::globals::JUMP_SQUAT_MAIN_UNIQ;
use crate::consts::globals::PREV_STATUS_KIND;
use smash::app::GroundCliffCheckKind;
use crate::ext::Vec3Ext;
use crate::ext::Vec2Ext;
use crate::consts::globals::CURRENT_FRAME;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_EscapeAir)]
unsafe extern "C" fn status_pre_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let pos = *PostureModule::pos(fighter.module_accessor);
    let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    
    // Check for ground underneath the player
    let lower_bound = Vector2f::new(pos.x, pos.y - 3.0);
    let ground_pos_any = &mut Vector2f::zero();
    let ground_pos_stage = &mut Vector2f::zero();
    let is_touch_any = GroundModule::line_segment_check(
        fighter.module_accessor,
        &Vector2f::new(pos.x, pos.y + 3.0),
        &lower_bound,
        &Vector2f::zero(),
        ground_pos_any,
        true,
    );

    let is_touch_stage = GroundModule::line_segment_check(
        fighter.module_accessor,
        &Vector2f::new(pos.x, pos.y + 3.0),
        &lower_bound,
        &Vector2f::zero(),
        ground_pos_stage,
        false,
    );

    // Stick input read
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);

    // Require diagonally downward input (not purely horizontal or purely downward)
    let is_diagonal_down = stick_y < -0.2 && stick_x.abs() > 0.2;

    // Determine if we can snap to ground
    let can_snap = is_diagonal_down && !(is_touch_any == 0 as *const *const u64 || (is_touch_stage != 0 as *const *const u64 && dir_y > 0.0));

    if prev_status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FALL
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH)
        && can_snap {
        
        GroundModule::attach_ground(fighter.module_accessor, true);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        PostureModule::set_pos(
            fighter.module_accessor,
            &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z),
        );
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    // Fallback to normal EscapeAir behavior
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_DISABLE, 
        false,
        false, 
        false, 
        0, 
        0, 
        0, 
        0, 
    );
    0.into()
}

//Escape Air
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_EscapeAir)]
unsafe extern "C" fn status_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
    } 
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_escapeair_main as *const () as _))
}

unsafe extern "C" fn status_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    /* START OF NEW ADDITION */
    //Rivals of Aether Momentum
    let anim_length = MotionModule::end_frame(boma);
    let start_cancel_frame = anim_length * 0.20; // 20% in
    let end_cancel_frame = anim_length * 0.60; // 60% in
    let final_cancel_frame = anim_length * 0.99; // 100% in

        if (start_cancel_frame..=end_cancel_frame).contains(&frame) {
            KineticModule::unable_energy_all(boma);
            KineticModule::clear_speed_all(boma);
        }
        if frame > end_cancel_frame {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        }

        if frame > final_cancel_frame {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_escapeair,
            status_escapeair,
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}