use smashline::L2CValue;
use smashline::L2CFighterCommon;
use smash2::app::WorkModule;
use crate::FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR;
use crate::FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY;
use crate::FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE;
use crate::imports::BuildImports::L2CFighterCommon_status_pre_Jump_sub_param;
use smash::lua2cpp::L2CFighterCommon_status_pre_AttackAir;
use smash::lib::lua_const::FIGHTER_TREADED_KIND_NO_REAC;
use crate::FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT;
use crate::FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT;
use smash::lib::lua_const::GROUND_CLIFF_CHECK_KIND_NONE;
use smash::app::GroundCliffCheckKind;
use smash::lib::lua_const::FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG;
use smash::lib::lua_const::FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE;
use smash::lib::lua_const::GROUND_CORRECT_KIND_AIR;
use smash::lib::lua_const::FIGHTER_KINETIC_TYPE_MOTION_FALL;
use smash::lib::lua_const::SITUATION_KIND_AIR;
use smash::app::SituationKind;
use smash2::app::StatusModule;
use smash2::app::FighterStatusModuleImpl;
use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_AttackAir)]
unsafe extern "C" fn status_pre_attackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

//Status Pre Jump Sub Param, handles momentum transfer
#[skyline::hook(replace = L2CFighterCommon_status_pre_Jump_sub_param)]
unsafe extern "C" fn status_pre_jump_sub_param(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue, param_5: L2CValue) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    call_original!(fighter, param_1, param_2, param_3, param_4, param_5)
}

