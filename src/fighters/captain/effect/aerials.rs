use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::EFFECT_FOLLOW_RND;

unsafe extern "C" fn effect_captain_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 2.5, 2, -148, -141, -3, 1.1, true, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 6, -5, -17, 5, 0, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 11, 14, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.5);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_captain_attackairn
    );
}