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
use crate::ext::BomaExt;
use crate::get_kind;
use crate::L2CAgent;
use crate::utils::clamp;

//=================================================================
//== EXTRA TRACTION
//=================================================================
/// Sets the extra traction flag depending on current speed and current status in order to prevent
/// the game feeling too slippery
unsafe extern "C" fn extra_traction(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
        let max_walk = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
        let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
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
        && fighter.global_table[CURRENT_FRAME].get_i32() > 0 {
            let added_traction: smash::phx::Vector3f = smash::phx::Vector3f {x: -1.0 * PostureModule::lr(boma) * ground_brake * speed_x.signum(), y: 0.0, z: 0.0};
            KineticModule::add_speed(boma, &added_traction);
        }
    }
}

pub fn install() {
    Agent::new("fighter")
    .on_line(Main, extra_traction)
	.install();
}