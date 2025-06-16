use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::L2CValue;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::app::SituationKind;
use smash::phx::Vector3f;
use smash::app::sv_math;
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
use crate::consts::globals::JUMP_SQUAT_MAIN_UNIQ;
use crate::consts::globals::PREV_STATUS_KIND;
use smash::app::GroundCliffCheckKind;
use crate::ext::Vec3Ext;
use crate::ext::Vec2Ext;
use crate::consts::globals::CURRENT_FRAME;

use crate::consts::globals::FLICK_Y;
use crate::consts::globals::STICK_Y;

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

//Status Jumpsquat Main, enables Wavedash out of Jumpsquat
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe extern "C" fn status_jumpsquat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    /* START OF NEW ADDITIONS */
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
    }
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    /* END OF NEW ADDITIONS */
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if !fighter.sub_transition_group_check_ground_item().get_bool() {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
            return 0.into();
        }
        if !fighter.sub_transition_specialflag_hoist().get_bool() {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_ptr()); callable(fighter).get_bool()} {
                    return 0.into();
                }
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
                    return 0.into();
                }
            }
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1daca540be));
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                    return 0.into();
                }
            }
        }
    }
    0.into()
}

//Status End Jumpsquat, clears flags
#[skyline::hook(replace = L2CFighterCommon_status_end_JumpSquat)]
unsafe extern "C" fn status_end_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_LandingLight_Main)]
pub unsafe fn status_landinglight_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret: i32 = 0;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let pass_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"));
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET)
        || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
            ret = 0.into();
        }
        let get_have_item_kind = ItemModule::get_have_item_kind(boma, 0);
        if get_have_item_kind == *ITEM_KIND_ASSIST {
            if !MotionModule::is_end(boma) {
                fighter.sub_landing_cancel_check_damage_face();
                ret = 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if !fighter.sub_landing_uniq_check_strans().get_bool() {
            if fighter.sub_landing_ground_check_common().get_bool() {
                ret = 1.into();
            }
            ret = 0.into();
        }
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status_kind) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
            ControlModule::reset_main_stick_x(boma);
            if GroundModule::is_passable_ground(boma)
            && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y
            && fighter.global_table[STICK_Y].get_f32() < pass_stick_y {
                fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
                return 1.into();
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    ret.into()
}

//Status End Landing
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_LandingLight)]
unsafe fn status_end_landinglight(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_cancel_damage_face();
    0.into()
}

//Landing Main Sub
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Landing_MainSub)]
unsafe fn status_landing_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret: i32 = 0;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let pass_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"));
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET)
        || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
            ret = 0.into();
        }
        let get_have_item_kind = ItemModule::get_have_item_kind(boma, 0);
        if get_have_item_kind == *ITEM_KIND_ASSIST {
            if !MotionModule::is_end(boma) {
                ret = 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if !fighter.sub_landing_uniq_check_strans().get_bool() {
            if fighter.sub_landing_ground_check_common().get_bool() {
                ret = 1.into();
            }
            ret = 0.into();
        }
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status_kind) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
			ControlModule::reset_main_stick_x(boma);
            if GroundModule::is_passable_ground(boma)
            && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y
            && fighter.global_table[STICK_Y].get_f32() < pass_stick_y {
                fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
                return 1.into();
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    ret.into()
}

//Status End Landing
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_Landing)]
unsafe fn status_end_landing(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_cancel_damage_face();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_jumpsquat_main,
            status_end_jumpsquat,
            status_pre_escapeair,
            status_landinglight_main,
            status_end_landinglight,
            status_landing_main_sub,
            status_end_landing

        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}