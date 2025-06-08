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
use smash::phx::Vector2f;
use smash::app::sv_system;
use smash::app::BattleObjectModuleAccessor;

//=================================================================
//== SHIELD DROP
//=================================================================
unsafe extern "C" fn shielddrop(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let sticky = ControlModule::get_stick_y(boma);
        if [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD].contains(&status_kind) &&  sticky <= -0.6875  && GroundModule::is_passable_ground(fighter.module_accessor){
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
		};
    };
}	

//=================================================================
//== SHIELD STOP
//=================================================================
unsafe extern "C" fn shield_stop(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
	if [
	*FIGHTER_STATUS_KIND_DASH,
	*FIGHTER_STATUS_KIND_TURN_DASH,
	].contains(&status) {
		if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD){
			fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
		}
	}
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, shield_stop)
	.on_line(Main, shielddrop)
	.install();
}