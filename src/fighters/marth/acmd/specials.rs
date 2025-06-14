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

unsafe extern "C" fn game_marth_specials3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_NONE), AttackDirectionAxis(*ATTACK_DIRECTION_NONE));
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 270, 30, 0, 40, 4.8, 0.0, 5.6, 14.0, Some(0.0), Some(7.0), Some(10.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 270, 30, 0, 40, 3.0, 0.0, 4.0, 19.0, Some(0.0), Some(7.0), Some(10.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

unsafe extern "C" fn effect_marth_specials3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 6, Hash40::new("sword1"), 0, 0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn sound_marth_specials3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_special_s03l"));
    }
}

unsafe extern "C" fn expression_marth_specials3lw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("marth")
    .acmd("game_specials3lw", game_marth_specials3lw, Priority::Low)
    .acmd("effect_specials3lw", effect_marth_specials3lw, Priority::Low)
    .acmd("sound_specials3lw", sound_marth_specials3lw, Priority::Low)
    .acmd("expression_specials3lw", expression_marth_specials3lw, Priority::Low)
    .install();
}