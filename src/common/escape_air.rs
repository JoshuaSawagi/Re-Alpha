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
use smash::app::sv_battle_object::entry_id;
use crate::PostureModule::lr;
use crate::utils::get_player_number;
use crate::KineticEnergy::get_speed_x;
use smash::app::sv_kinetic_energy;
use crate::consts::globals::PREV_SITUATION_KIND;
use smash::lua2cpp::L2CFighterCommon_uniq_process_JumpSquat_exec_status_param;

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

/*unsafe extern "C" fn status_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let anim_length = MotionModule::end_frame(fighter.module_accessor);

    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        // Setting a window based on animation length
        let start_airdodge_length = anim_length * 0.16;
        let end_airdodge_length = anim_length * 0.3;

        // Apply momentum clearing only in the 18%–29% window
        if (start_airdodge_length..=end_airdodge_length).contains(&frame) {
            KineticModule::unable_energy_all(boma);
            KineticModule::clear_speed_all(boma);
        }
        
        if start_airdodge_length > end_airdodge_length {
            // Restore normal fall behavior
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    0.into()
}*/

unsafe extern "C" fn status_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let is_directional = WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);
    let anim_length = MotionModule::end_frame(fighter.module_accessor);
    let is_neutral = !is_directional;

    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
        let start_airdodge_length = anim_length * 0.16;
        let end_airdodge_length = anim_length * 0.3;
    if is_directional {
        // Applies to directional
        if frame <= end_airdodge_length {
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.0001);
        }
        if (start_airdodge_length..=end_airdodge_length).contains(&frame) {
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.00001);
            KineticModule::unable_energy_all(boma);
            KineticModule::clear_speed_all(boma);
        }
        if start_airdodge_length > end_airdodge_length {
            KineticModule::change_kinetic(boma, *FIGHTER_STATUS_KIND_FALL);
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    } else if is_neutral {
        let start_neutral_length = anim_length * 0.11;
        let end_neutral_length = anim_length * 0.3;
        // Applies to neutral
        if (start_neutral_length..=end_airdodge_length).contains(&frame) {
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.00001);
            KineticModule::unable_energy_all(boma);
            KineticModule::clear_speed_all(boma);
        }
        if frame > end_airdodge_length {
            KineticModule::change_kinetic(boma, *FIGHTER_STATUS_KIND_FALL);
            KineticModule::resume_energy_all(boma);
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    0.into()
}

/*pub const CUSTOM_ROUTINE:        i32 = 0x2B;
const WORK_ID_ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x1000;
const WORK_ID_JUMP_SPEED_RATIO: i32 = 0x1001;
const WORK_ID_JUMP_SQUAT_FRAME: i32 = 0x1002;
const WORK_ID_JUMPSQUAT_VELOCITY: i32 = 0x1003;
const WORK_ID_CURRENT_MOMENTUM: i32 = 0x1005;
const WORK_ID_CURRENT_MOMENTUM_SPECIALS: i32 = 0x1006;


#[skyline::hook(replace = L2CFighterCommon_uniq_process_JumpSquat_exec_status_param)]
unsafe extern "C" fn uniq_process_JumpSquat_exec_status_param(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    let should_check = if fighter.global_table[CUSTOM_ROUTINE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[CUSTOM_ROUTINE].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter);
            true
        } else {
            true
        }
    } else { true };
    if should_check {
        fighter.sub_jump_squat_uniq_check_sub(L2CValue::I32(*FIGHTER_STATUS_JUMP_FLAG_BUTTON));
        fighter.sub_jump_squat_uniq_check_sub_mini_attack();
    }

    let mot = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    /*let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0 || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD))
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N == 0 {
        if !(fighter.kind() == *FIGHTER_KIND_PICKEL 
        && fighter.is_prev_status_one_of(&[*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT])) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
        }
    }*/
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(mot));
    if frame >= end_frame {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        let situation_kind = fighter.global_table[SITUATION_KIND].clone();
        fighter.global_table[PREV_SITUATION_KIND].assign(&situation_kind);
        /*if VarModule::is_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT) {
            if fighter.global_table[STICK_Y].get_f32() <= 0.2
            {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
                // change kinetic/ground properties for wavedash
                //GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            } else {
                VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
                // change kinetic properties for rising airdodge
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        } 
        else {*/
            // change kinetic/ground properties for jump
            //VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_SQUAT, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
        //}
    }
    else {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        println!("js_vel: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
        WorkModule::set_float(boma, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN), WORK_ID_JUMPSQUAT_VELOCITY);
        WorkModule::set_float(boma, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN), WORK_ID_CURRENT_MOMENTUM);
        WorkModule::set_float(boma, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN), WORK_ID_CURRENT_MOMENTUM_SPECIALS);
    }
}*/

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_escapeair,
            //uniq_process_JumpSquat_exec_status_param
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}