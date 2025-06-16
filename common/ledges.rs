use smash::hash40;
use crate::get_kind;
use crate::WorkModule;
use crate::GroundModule;
use crate::StatusModule;
use crate::MotionModule;
use smash::phx::Vector2f;
use smash::phx::Vector3f;
use crate::KineticModule;
use smash::lib::lua_const::*;
use crate::utils::get_player_number;
use crate::app;

static mut LEDGE_POS: [Vector3f; 8] = [smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}; 8];

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::entry_cliff)]
pub unsafe fn entry_cliff_hook(fighter: &mut app::BattleObjectModuleAccessor) {
    let entry_id = get_player_number(fighter);
    LEDGE_POS[entry_id] = GroundModule::hang_cliff_pos_3f(fighter);
    original!()(fighter)
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::leave_cliff)]
pub unsafe fn leave_cliff_hook(fighter: &mut app::BattleObjectModuleAccessor) {
    let entry_id = get_player_number(fighter);
    LEDGE_POS[entry_id] = smash::phx::Vector3f { x: 0.0, y: 0.0, z:0.0 };
    original!()(fighter)
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::can_entry_cliff)]
pub unsafe fn can_entry_cliff_hook(fighter: &mut app::BattleObjectModuleAccessor) -> u64 {
    let entry_id = WorkModule::get_int(fighter, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let pos = GroundModule::hang_cliff_pos_3f(fighter);
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter);
	

	// De-occupy ledge if not on ledge anymore
    if ![*FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT,
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3].contains(&status_kind) {
        LEDGE_POS[entry_id] = smash::phx::Vector3f { x: 0.0, y: 0.0, z:0.0 };
    }

	for i in 0..8 {
		i as usize;
		if i == entry_id || LEDGE_POS[i].x == 0.0 {
			continue;
		}

		if pos.x == LEDGE_POS[i].x && pos.y == LEDGE_POS[i].y {
			return 0 as u64;
		}
	}

    original!()(fighter)
}

pub fn install() {
    skyline::install_hooks!(
        entry_cliff_hook,
        leave_cliff_hook,
        can_entry_cliff_hook
    );
}