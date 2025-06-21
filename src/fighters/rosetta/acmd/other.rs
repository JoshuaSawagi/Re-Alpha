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
use smash::app::*;

pub fn install() {
	Agent::new("rosetta")
    /*example
    .on_line("game_attack11", game_rosa_attack11, Priority::Low)*/
	.install();
}