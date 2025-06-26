use smash::lib::lua_const::*;
use smash::phx::Vector2f;
use smashline::Main;
use smash::app::sv_system;
use std::convert::TryInto;
use smash::phx::*;
use smashline::Agent;
use smash::app::{self, lua_bind::*, utility, BattleObjectModuleAccessor};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::app::GroundCorrectKind;
use smashline::L2CValue;
use smash::app::FighterUtil::get_ground_correct_kind_air_trans;
use crate::utils::is_ready_go;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_DOUBLE_TRACTION;
use smash::app::sv_kinetic_energy::get_speed_x;
use crate::ext::BomaExt;
use crate::utils::compare_cat;
use crate::consts::FIGHTER_INSTANCE_WORK_FLOAT_GLIDE_TOSS_DIR;
use crate::utils::get_player_number;

//=================================================================
//== GLIDE TOSS (no VarModule version)
//=================================================================
static mut can_glide_toss: [bool; 8] = [false; 8];

unsafe extern "C" fn glide_toss(fighter: &mut L2CFighterCommon) {
	unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = StatusModule::status_kind(boma);
		let id = get_player_number(boma);
		let prev_status = StatusModule::prev_status_kind(boma, 0);
		let facing = PostureModule::lr(boma);
		if [*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&status_kind) {
			if MotionModule::frame(boma) <= 6.0 {
				can_glide_toss[id] = true;
				WorkModule::set_float(boma, facing, *FIGHTER_INSTANCE_WORK_FLOAT_GLIDE_TOSS_DIR);
			} else {
				can_glide_toss[id] = false;
			}
		}
		if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW {
			let roll_dir = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_FLOAT_GLIDE_TOSS_DIR);
			if (prev_status == *FIGHTER_STATUS_KIND_ESCAPE_F) && can_glide_toss[id] {
				let motion_value: f32 = 2.8 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
				let motion_vec = Vector3f { x: motion_value * roll_dir, y: 0.0, z: 0.0 };
				KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
			}
			if (prev_status == *FIGHTER_STATUS_KIND_ESCAPE_B) && can_glide_toss[id] {
				let motion_value: f32 = 2.8 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
				let motion_vec = Vector3f { x: motion_value * roll_dir * -1.0, y: 0.0, z: 0.0 };
				KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
			}
		}
	}
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, glide_toss)
	.install();
}