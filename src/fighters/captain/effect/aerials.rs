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
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 10.75, 10.0, -20.0, 0.0, 0.0, 0.7, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 2.0, 7.75, 8.0, 0.0, 0.0, 0.0, 1.3, true);
	    macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("captain_at_thrust"), Hash40::new("top"), 0.0, 5.75, 2.0, -20.0, 10.0, 0.0, 0.85, true, 0.5);
	    macros::LAST_EFFECT_SET_RATE(agent, 1);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_captain_attackairn
    );
}